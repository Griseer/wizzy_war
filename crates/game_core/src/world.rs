// crates/game_core/src/world.rs
use crate::PlayerId;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
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
        self.players
            .insert(player_id, PlayerState { x: 0.0, y: 0.0 });
    }
    pub fn move_player(&mut self, player_id: PlayerId, dx: f32, dy: f32) {
        if let Some(player) = self.players.get_mut(&player_id) {
            player.x += dx;
            player.y += dy;
        }
    }
}
