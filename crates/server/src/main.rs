// server

use bevy::prelude::*;
use game_core::{MoveCommand, Player, PlayerId, Position};
use shared::TICK_RATE;
use std::collections::BTreeMap;

#[derive(Resource, Default)]
struct SnapshotHistory {
    states: BTreeMap<u64, Player>,
}

#[derive(Resource, Default)]
struct CommandHistory {
    commands: BTreeMap<u64, MoveCommand>,
}
#[derive(Resource, Default)]
struct TickCounter {
    current: u64,
}

#[derive(Resource)]
struct PlayerResource(pub Player);

fn main() {
    App::new()
        .add_plugins(MinimalPlugins)
        .insert_resource(Time::<Fixed>::from_hz(TICK_RATE.into()))
        .insert_resource(PlayerResource(Player {
            id: PlayerId(1),
            position: Position { x: 0.0, y: 0.0 },
        }))
        .insert_resource(CommandHistory::default())
        .insert_resource(TickCounter::default())
        .insert_resource(SnapshotHistory::default())
        .add_systems(FixedUpdate, simulation_step)
        .run();
}

fn simulation_step(
    mut player: ResMut<PlayerResource>,
    mut history: ResMut<CommandHistory>,
    mut snapshots: ResMut<SnapshotHistory>,
    mut tick: ResMut<TickCounter>,
) {
    let current_tick = tick.current;

    // Guardar snapshot ANTES de simular
    snapshots.states.insert(current_tick, player.0.clone());

    let cmd = history.commands.remove(&current_tick);
    game_core::simulate_player(&mut player.0, cmd);

    tick.current += 1;
}
