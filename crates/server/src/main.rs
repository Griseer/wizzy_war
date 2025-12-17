// server
use bevy::prelude::*;
use game_core::{MoveCommand, Player, PlayerId, Position};
use net::{ClientMessage, ServerMessage};
use shared::{ROLLBACK_WINDOW, TICK_RATE};
use std::collections::BTreeMap;
use std::net::UdpSocket;

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

#[derive(Resource)]
struct ServerSocket(UdpSocket);

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:4000").unwrap();
    socket.set_nonblocking(true).unwrap();
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
        .insert_resource(ServerSocket(socket))
        .add_systems(FixedUpdate, recv_commands)
        .add_systems(FixedUpdate, simulation_step)
        .run();
}

fn simulation_step(
    mut player: ResMut<PlayerResource>,
    mut history: ResMut<CommandHistory>,
    mut snapshots: ResMut<SnapshotHistory>,
    socket: Res<ServerSocket>,
    mut tick: ResMut<TickCounter>,
) {
    let current_tick = tick.current;

    // Guardar snapshot ANTES de simular
    snapshots.states.insert(current_tick, player.0.clone());

    let cmd = history.commands.remove(&current_tick);
    game_core::simulate_player(&mut player.0, cmd);

    tick.current += 1;
    let min_tick = tick.current.saturating_sub(ROLLBACK_WINDOW);

    snapshots.states.retain(|&t, _| t >= min_tick);

    history.commands.retain(|&t, _| t >= min_tick);
    let msg = ServerMessage::Snapshot {
        tick: current_tick,
        player: player.0.clone(),
    };

    let bytes = shared::serialize(&msg);
    let _ = socket.0.send_to(&bytes, "127.0.0.1:4001");
}

fn rollback_to_tick(
    target_tick: u64,
    current_tick: u64,
    player: &mut Player,
    snapshots: &SnapshotHistory,
    commands: &CommandHistory,
) {
    // 1. Restaurar snapshot
    let snapshot = snapshots
        .states
        .get(&target_tick)
        .expect("Snapshot missing")
        .clone();

    *player = snapshot;

    // 2. Re-simular hasta el tick actual
    for tick in target_tick..current_tick {
        let cmd = commands.commands.get(&tick).copied();
        game_core::simulate_player(player, cmd);
    }
}

fn recv_commands(socket: Res<ServerSocket>, mut history: ResMut<CommandHistory>) {
    let mut buf = [0u8; 1024];

    while let Ok((len, _addr)) = socket.0.recv_from(&mut buf) {
        let msg: ClientMessage = shared::deserialize(&buf[..len]);

        if let ClientMessage::Command(cmd) = msg {
            history.commands.insert(cmd.tick, cmd.command);
        }
    }
}
