use std::net::SocketAddr;
use shared::input::{InputFrame, Buttons};
use shared::ids::PlayerId;
use state::PlayerState;

pub mod input;

pub mod state;


pub struct Player {
    id: PlayerId,
    state: PlayerState,
    last_processed_tick: u64,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Player {
            id,
            state: state::PlayerState::new(),
            last_processed_tick: 0
        }
    }

    pub fn apply_input(&mut self, frame:&InputFrame){
        // 1️⃣ ignorar inputs viejos o repetidos
        if frame.tick <= self.last_processed_tick {
            return;
        }

        self.last_processed_tick = frame.tick;

        // 2️⃣ movimiento por click
        if frame.buttons.contains(Buttons::MOVE_CLICK) {
            if let Some(target) = frame.move_target {
                self.state.move_target = Some(target);
            }
        }

    }

}
