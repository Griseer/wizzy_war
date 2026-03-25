
// crates/game_core/src/world.rs


use slotmap::{SlotMap, new_key_type};

use crate::model::{events::EventQueue, player::Player, spells::{Projectile, Spell, SpellId}};
use shared::{ids::PlayerId, input::InputFrame};
use std::collections::HashMap;





pub struct World {
    pub players: HashMap<PlayerId, Player>,

    pub projectiles: Vec<Projectile>,

    pub events:EventQueue


    

}


impl World {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
            projectiles :Vec::new(),
            events:EventQueue::new()
        }
    }

    pub fn add_player(&mut self, player:Player) {
        self.players.insert(player.id, player);
    }

    pub fn apply_input(&mut self, player_id: &PlayerId, frame:InputFrame){
        if let Some(player) = self.players.get_mut(player_id) {
            player.apply_input(&frame);
        }
    }



}
