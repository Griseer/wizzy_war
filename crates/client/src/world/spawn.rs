//client/world/spawn.rs

use bevy::prelude::*;
use std::collections::HashSet;

use crate::state::ClientState;
use crate::world::components::{PlayerEntity, RemotePlayer};

pub fn spawn_players_system(
    mut commands: Commands,
    state: Res<ClientState>,
    query: Query<&PlayerEntity>,
) {
    // jugadores que ya existen en ECS
    let mut existing = HashSet::new();
    for player in query.iter() {
        existing.insert(player.id);
    }

    // jugadores que vienen del server
    for (player_id, _) in state.server_players.iter() {
        if existing.contains(player_id) {
            continue;
        }

        commands.spawn((
            PlayerEntity { id: *player_id },
            RemotePlayer,
            Transform::default(),
            GlobalTransform::default(),
        ));

        info!("Spawned player entity {:?}", player_id);
    }
}
