use bevy::prelude::*;
use game_core::PlayerId;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct RemoteWorld {
    pub players: HashMap<PlayerId, Vec2>,
}
