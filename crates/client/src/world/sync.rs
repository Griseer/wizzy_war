// client/src/world/sync.rs

use bevy::prelude::*;

use crate::state::ClientState;
use crate::world::components::PlayerEntity;

pub fn sync_players_from_snapshot_system(
    state: Res<ClientState>,
    mut query: Query<(&PlayerEntity, &mut Transform)>,
) {
    // si no hubo snapshot todavía, no hacemos nada
    if state.server_players.is_empty() {
        return;
    }

    for (player_entity, mut transform) in query.iter_mut() {
        if let Some(snapshot) = state.server_players.get(&player_entity.id) {
            transform.translation.x = snapshot.position.x;
            transform.translation.y = snapshot.position.y;
            transform.translation.z = 0.0;
        }
    }
}