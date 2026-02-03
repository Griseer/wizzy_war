//game_core/src/player.mod.rs

use shared::ids::PlayerId;
use shared::input::{Buttons, InputFrame};
use shared::tick::InputTick;
use state::PlayerState;
use std::net::SocketAddr;

pub mod input;

pub mod state;

pub struct Player {
    pub id: PlayerId,
    pub state: PlayerState,
    pub last_processed_tick: InputTick,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Player {
            id,
            state: state::PlayerState::new(),
            last_processed_tick: InputTick(0),
        }
    }

    pub fn apply_input(&mut self, frame: &InputFrame) {
        // ignorar inputs viejos o repetidos
        if frame.tick <= self.last_processed_tick {
            return;
        }

        self.last_processed_tick = frame.tick;

        // 2️⃣ movimiento por click
        if frame.buttons.contains(Buttons::MOVE_TO) {
            self.state.move_target = Some(frame.aim_target);
        }

        self.state.aim_dir = frame.aim_target.normalized();
    }
}
