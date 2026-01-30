use bevy::prelude::*;

pub mod components;
pub mod spawn;
pub mod sync;

use spawn::spawn_players_system;
use sync::sync_players_from_snapshot_system;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,
            (spawn_players_system, 
            sync_players_from_snapshot_system));
    }
}