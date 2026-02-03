use game_core::player::Player;
use net::client::message::ClientInputMessage;
use net::server::message::ServerMessage;
use shared::ids::PlayerId;
use shared::input::InputFrame;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::net::SocketAddr;

use crate::simulation::snapshot::ServerSnapshot;
use crate::simulation::tick::run;
use shared::tick::{InputTick, ServerTick};

pub struct ServerState {
    next_player_id: u64,
    pub tick: ServerTick,
    pub addr_players: HashMap<SocketAddr, PlayerId>,
    pub players: HashMap<PlayerId, Player>,
    pub input_buffers: HashMap<PlayerId, InputBuffer>,
    pub last_snapshot: ServerSnapshot,
    pub tick_rate: u64,
}

pub struct InputBuffer {
    pub last_processed_tick: InputTick,
    pub frames: VecDeque<InputFrame>,
}

impl InputBuffer {
    fn new() -> Self {
        InputBuffer {
            last_processed_tick: InputTick(0),
            frames: VecDeque::new(),
        }
    }
}

impl ServerState {
    pub fn new(tickRate: u64) -> Self {
        ServerState {
            next_player_id: 1,
            tick: ServerTick(0),
            addr_players: HashMap::new(),
            players: HashMap::new(),
            input_buffers: HashMap::new(),
            last_snapshot: ServerSnapshot {
                tick: ServerTick(0),
                players: Vec::new(),
            },
            tick_rate: tickRate,
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
        self.players.insert(player_id, new_player);
        self.input_buffers.insert(player_id, InputBuffer::new());
        player_id
    }

    pub fn handle_input(&mut self, addr: SocketAddr, msg: ClientInputMessage) {
        // 1️⃣ ¿Quién envía?
        let player_id = match self.addr_players.get(&addr) {
            Some(id) => *id,
            None => {
                // input de alguien no registrado
                return;
            }
        };

        // 2️⃣ Obtener o crear buffer
        let buffer = self
            .input_buffers
            .entry(player_id)
            .or_insert_with(InputBuffer::new);

        // 3️⃣ Procesar frames
        for frame in msg.inputs {
            // 🔒 descartar inputs viejos o duplicados
            if frame.tick <= buffer.last_processed_tick {
                continue;
            }

            // 🔒 evitar duplicados (mismo tick)
            if buffer.frames.iter().any(|f| f.tick == frame.tick) {
                continue;
            }

            buffer.frames.push_back(frame);
        }

        // 4️⃣ Mantener orden por tick (por si llegan fuera de orden)
        buffer.frames.make_contiguous().sort_by_key(|f| f.tick);

        // (opcional) limitar tamaño
        const MAX_BUFFERED_INPUTS: usize = 64;
        while buffer.frames.len() > MAX_BUFFERED_INPUTS {
            buffer.frames.pop_front();
        }
    }

    pub fn simulate_tick(&mut self) {
        run(self);
    }
}
