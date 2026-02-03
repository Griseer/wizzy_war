// client/src/world/components.rs

use bevy::prelude::*;
use shared::ids::PlayerId;

#[derive(Component)]
pub struct PlayerEntity {
    pub id: PlayerId,
}

#[derive(Component)]
pub struct RemotePlayer;
