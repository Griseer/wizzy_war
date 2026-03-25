use game_core::model::player::Player;
use game_core::model::world::World;

use net::client::message::ClientInputMessage;
use shared::ids::PlayerId;
use shared::input::InputFrame;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::net::SocketAddr;

use crate::tick::snapshot::ServerSnapshot;
use shared::tick::{Tick};

pub struct ServerState {
    next_player_id: u64,
    pub tick: Tick,
    pub world: World,
    pub addr_players: HashMap<SocketAddr, PlayerId>,
    pub input_buffers: HashMap<PlayerId, InputBuffer>,
    pub last_snapshot: ServerSnapshot,
    pub tick_rate: u64,
}

pub struct InputBuffer {
    pub last_processed_tick: Tick,
    pub frames: VecDeque<InputFrame>,
}

impl InputBuffer {
    fn new() -> Self {
        InputBuffer {
            last_processed_tick: Tick(0),
            frames: VecDeque::new(),
        }
    }
}

impl ServerState {
    pub fn new(tick_rate: u64) -> Self {
        ServerState {
            next_player_id: 1,
            tick: Tick(0),
            world:World::new(),
            addr_players: HashMap::new(),
            input_buffers: HashMap::new(),
            last_snapshot: ServerSnapshot {
                tick: Tick(0),
                players: Vec::new(),
            },
            tick_rate,
        }
    }

    pub fn add_player(&mut self, addr: SocketAddr) -> PlayerId {
        if let Some(&id) = self.addr_players.get(&addr) {
            // 🔒 ya existe → NO crear otro
            return id;
        }
        let player_id = PlayerId(self.next_player_id as u64);
        let new_player = Player::new(player_id);
        self.next_player_id += 1;
        self.addr_players.insert(addr, player_id);
        self.world.add_player(new_player);
        self.input_buffers.insert(player_id, InputBuffer::new());
        player_id
    }

    pub fn handle_input(&mut self, addr: SocketAddr, msg: ClientInputMessage) {
        let player_id = match self.addr_players.get(&addr) {
            Some(id) => *id,
            None => {
                // input de alguien no registrado
                return;
            }
        };

        let buffer = self
            .input_buffers
            .entry(player_id)
            .or_insert_with(InputBuffer::new);

    
        for frame in msg.inputs {

            //descartar inputs viejos o duplicados
            if frame.tick <= buffer.last_processed_tick {
                continue;
            }

            buffer.frames.push_back(frame);
        }

        // ordenar el buffer
        buffer.frames.make_contiguous().sort_by_key(|f| f.tick);

        // (opcional) limitar tamaño
        const MAX_BUFFERED_INPUTS: usize = 64;
        while buffer.frames.len() > MAX_BUFFERED_INPUTS {
            buffer.frames.pop_front();
        }
    }

}
