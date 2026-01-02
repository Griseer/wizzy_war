// crates/game_core/src/world.rs
use crate::player::player_state::PlayerState;
use crate::PlayerId;
use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct WorldState {
    pub players: HashMap<PlayerId, PlayerState>,
}

impl WorldState {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub fn add_player(&mut self, player_id: PlayerId) {
        self.players.insert(player_id, PlayerState::new());
    }
    pub fn move_player(&mut self, player_id: PlayerId, dx: f32, dy: f32) {
        if let Some(player) = self.players.get_mut(&player_id) {
            player.position.x += dx;
            player.position.y += dy;
        }
    }
}
