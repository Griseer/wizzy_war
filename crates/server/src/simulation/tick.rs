//server/src/simulation/tick.rs

use crate::state::ServerState;

use crate::net::send::broadcast_snapshot;
use crate::simulation::snapshot::{ServerPlayerSnapshot, ServerSnapshot, build_snapshot};
use crate::simulation::step::simulate_players;
use net::server::message::{PlayerSnapshot, ServerMessage};
use shared::tick::InputTick;

pub fn run(state: &mut ServerState) {
    // 1️⃣ aplicar inputs de este tick
    apply_inputs(state);

    // 2️⃣ simular mundo
    simulate_players(state);

    state.last_snapshot = build_snapshot(&state);

    // 3️⃣ limpiar buffers
    cleanup(state);

    // ⬅️ acá se avanza el tiempo
    state.tick += 1;
}

fn apply_inputs(state: &mut ServerState) {
    // procesa inputs del tick actual
    let tick = state.tick;

    for (player_id, buffer) in state.input_buffers.iter_mut() {
        // procesar todos los inputs para este tick

        while let Some(frame) = buffer.frames.front() {
            if frame.tick <= buffer.last_processed_tick {
                buffer.frames.pop_front();
                continue;
            }
            // ✔️ este input es nuevo → procesar
            let frame = buffer.frames.pop_front().unwrap();
            buffer.last_processed_tick = frame.tick;

            // aplicar intención
            if let Some(player) = state.players.get_mut(player_id) {
                player.apply_input(&frame);
            }
        }
    }
}

fn cleanup(state: &mut ServerState) {
    // borra inputs viejos, etc
    let tick = state.tick;

    for buffer in state.input_buffers.values_mut() {
        // eliminar inputs muy viejos (lag extremo o replay)
        while let Some(frame) = buffer.frames.front() {
            if frame.tick < InputTick(tick.0 - 5) {
                buffer.frames.pop_front();
            } else {
                break;
            }
        }
    }
}
