use game_core::{PlayerId, WorldState};
use net::{ClientMessage, ServerMessage};
use std::time::{Duration, Instant};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

struct ServerState {
    next_player_id: u32,
    clients: HashMap<SocketAddr, PlayerId>,
    world: WorldState,
}

impl ServerState {
    fn new() -> Self {
        Self {
            next_player_id: 1,
            clients: HashMap::new(),
            world: WorldState::new(),
        }
    }

    fn allocate_player_id(&mut self) -> PlayerId {
        let id = PlayerId(self.next_player_id);
        self.next_player_id += 1;
        id
    }
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:4000")?;
    socket.set_nonblocking(true)?;
    println!("UDP server listening on 127.0.0.1:4000");

    let mut state = ServerState::new();
    let mut buffer = [0u8; 1024];
    let mut last_snapshot = Instant::now();

    loop {
        match socket.recv_from(&mut buffer) {
            Ok((len, addr)) => {
                handle_packet(&socket, &buffer[..len], addr, &mut state)?;
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // no data, seguimos
            }
            Err(e) => return Err(e),
        }

        if last_snapshot.elapsed() >= Duration::from_millis(100) {
            send_snapshot(&socket, &state)?;
            last_snapshot = Instant::now();
        }
    }
}

fn handle_packet(
    socket: &UdpSocket,
    data: &[u8],
    addr: SocketAddr,
    state: &mut ServerState,
) -> std::io::Result<()> {
    let msg = match ClientMessage::decode(data) {
        Some(m) => m,
        None => return Ok(()),
    };

    match msg {
        ClientMessage::Join => {
            let player_id = if let Some(id) = state.clients.get(&addr) {
                *id
            } else {
                let id = state.allocate_player_id();
                state.clients.insert(addr, id);
                state.world.add_player(id);
                println!("Player {:?} connected from {}", id, addr);
                id
            };

            let mut buffer = Vec::new();
            ServerMessage::Welcome { player_id }.encode(&mut buffer);
            socket.send_to(&buffer, addr)?;
        }
        ClientMessage::Move { dx, dy } => {
            if let Some(player_id) = state.clients.get(&addr) {
                state.world.move_player(*player_id, dx, dy);
            }
        }
    }
    Ok(())
}
fn send_snapshot(socket: &UdpSocket, state: &ServerState) -> std::io::Result<()> {
    let players = state
        .world
        .players
        .iter()
        .map(|(id, p)| (*id, p.clone()))
        .collect::<Vec<_>>();

    let mut buffer = Vec::new();
    ServerMessage::Snapshot { players }.encode(&mut buffer);

    for addr in state.clients.keys() {
        socket.send_to(&buffer, addr)?;
    }

    Ok(())
}
