use shared::math::Vec2f;

use crate::state::ServerState;

pub fn simulate_players(state: &mut ServerState) {
    const DELTA: f32 = 1.0 / 30.0; // 60 ticks por segundo
    const ARRIVAL_EPSILON: f32 = 0.05;

    for player in state.players.values_mut() {
        
        let Some(target) = player.state.move_target else {
            continue;
        };


        let pos = player.state.position;
        let dir = target - pos;
        let distance = dir.length();

        // 📍 llegó al destino
        if distance <= ARRIVAL_EPSILON {
            player.state.position = target;
            player.state.move_target = None;
            continue;
        }

        // 🏃 avanzar hacia el target
        let max_step = player.state.move_speed * DELTA;

        if distance <= max_step {
            // llega en este tick
            player.state.position = target;
            player.state.move_target = None;
        } else {
            // avanza parcialmente
            let step = dir.normalized() * max_step;
            player.state.position =  player.state.position + step;
        }
    }


}