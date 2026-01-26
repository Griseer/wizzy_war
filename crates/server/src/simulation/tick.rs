use crate::state::ServerState;

use crate::simulation::step::simulate_players;



pub fn run(state: &mut ServerState){
  // 1️⃣ aplicar inputs de este tick
    apply_inputs(state);

    // 2️⃣ simular mundo
    simulate_players(state);

    // 3️⃣ limpiar buffers
    cleanup(state);

    state.tick += 1;
}


fn apply_inputs(state: &mut ServerState) {
    // procesa inputs del tick actual
    let tick = state.tick;

    for (player_id, buffer) in state.input_buffers.iter_mut() {
        // procesar todos los inputs para este tick
        while let Some(frame) = buffer.frames.front() {
            if frame.tick < tick {
                // viejo → descartable
                buffer.frames.pop_front();
                continue;
            }

            if frame.tick > tick {
                // futuro → esperar
                break;
            }

            // frame.tick == tick ✔
            let frame = buffer.frames.pop_front().unwrap();
            buffer.last_processed_tick = tick;

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
            if frame.tick + 5 < tick {
                buffer.frames.pop_front();
            } else {
                break;
            }
        }
    }
}