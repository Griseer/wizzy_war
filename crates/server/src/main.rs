use game_core::PlayerId;
use net::{ClientMessage, ServerMessage};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
};

struct ServerState {
    next_player_id: u32,
    clients: HashMap<SocketAddr, PlayerId>,
}

impl ServerState {
    fn new() -> Self {
        Self {
            next_player_id: 1,
            clients: HashMap::new(),
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
    println!("UDP server listening on 127.0.0.1:4000");

    let mut state = ServerState::new();
    let mut buffer = [0u8; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buffer)?;
        handle_packet(&socket, &buffer[..len], addr, &mut state)?;
    }
}

fn handle_packet(
    socket: &std::net::UdpSocket,
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
            let player_id = state.clients.entry(addr).or_insert_with(|| {
                let id = PlayerId(state.next_player_id);
                state.next_player_id += 1;
                println!("Player {:?} connected from {}", id, addr);
                id
            });

            let mut buffer = Vec::new();
            ServerMessage::Welcome {
                player_id: *player_id,
            }
            .encode(&mut buffer);
            socket.send_to(&buffer, addr)?;
        }
    }

    Ok(())
}
