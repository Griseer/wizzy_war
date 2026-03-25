pub mod snapshot;
pub mod step;



//server/src/simulation/tick.rs

use crate::state::ServerState;
use crate::tick::snapshot::{build_snapshot};
use game_core::simulate_world;
use shared::tick::{self, Tick};

pub fn run_tick(state: &mut ServerState, dt:f32) {

    let tick = state.tick.0;

    apply_inputs(state);

    simulate_world(&mut state.world, dt, tick);

    state.last_snapshot = build_snapshot(&state);

    cleanup(state);

    
    state.tick += 1;
    
}

fn apply_inputs(state: &mut ServerState) {
    let current_tick = state.tick;

    for (player_id, buffer) in state.input_buffers.iter_mut() {
        if let Some(frame) = buffer.frames.front() {

            if frame.tick <= buffer.last_processed_tick {
                buffer.frames.pop_front();
                continue;
            }

            if frame.tick < current_tick {
                let frame = buffer.frames.pop_front().unwrap();
                buffer.last_processed_tick = frame.tick;

                state.world.apply_input(player_id, frame);
            }

            // si es futuro → no hacer nada
        }
    }
}

fn cleanup(state: &mut ServerState) {
    let tick = state.tick;

    for buffer in state.input_buffers.values_mut() {
        // eliminar inputs muy viejos (lag extremo o replay)
        while let Some(frame) = buffer.frames.front() {
            if frame.tick < Tick(tick.0 - 5) {
                buffer.frames.pop_front();
            } else {
                break;
            }
        }
    }
}
