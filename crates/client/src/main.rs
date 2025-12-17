//CLIENT

use bevy::{input::ButtonInput, prelude::*};
use game_core::{InputState, MoveCommand, Player, PlayerId, Position, TickedCommand};
use net::{ClientMessage, ServerMessage};
use shared::{ROLLBACK_WINDOW, TICK_RATE};
use std::collections::BTreeMap;
use std::net::UdpSocket;

#[derive(Resource)]
struct ClientSocket(UdpSocket);

#[derive(Resource)]
struct ServerState {
    player: Player,
    tick: u64,
}

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

#[derive(Component)]
struct PlayerVisual;

#[derive(Component)]
struct VisualSmoothing {
    target: Vec2,
}

fn main() {
    let socket = UdpSocket::bind("127.0.0.1:4001").unwrap();
    socket.set_nonblocking(true).unwrap();
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Time::<Fixed>::from_hz(TICK_RATE.into()))
        .insert_resource(PlayerResource(Player {
            id: PlayerId(1),
            position: Position { x: 0.0, y: 0.0 },
        }))
        .insert_resource(CommandHistory::default())
        .insert_resource(TickCounter::default())
        .insert_resource(SnapshotHistory::default())
        .insert_resource(ServerState {
            player: Player {
                id: PlayerId(0),
                position: Position { x: 0.0, y: 0.0 },
            },
            tick: 0,
        })
        .insert_resource(ClientSocket(socket))
        .add_systems(FixedUpdate, simulation_step)
        .add_systems(Update, input_system)
        .add_systems(Update, sync_player_visual)
        .add_systems(Startup, (setup_camera, spawn_player_visual))
        .add_systems(Update, rollback_test_system)
        .add_systems(Update, reconciliation_system)
        .add_systems(Update, visual_smoothing_system)
        .add_systems(FixedUpdate, recv_snapshots)
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

    let min_tick = tick.current.saturating_sub(ROLLBACK_WINDOW);

    snapshots.states.retain(|&t, _| t >= min_tick);

    history.commands.retain(|&t, _| t >= min_tick);
}

fn reconciliation_system(
    mut player: ResMut<PlayerResource>,
    server: Res<ServerState>,
    snapshots: Res<SnapshotHistory>,
    commands: Res<CommandHistory>,
    tick: Res<TickCounter>,
) {
    let server_tick = server.tick;

    if let Some(client_snapshot) = snapshots.states.get(&server_tick) {
        let diff = position_distance(client_snapshot.position, server.player.position);

        if diff > 0.01 {
            info!(
                "DESYNC @ tick {} (client {:.2},{:.2} vs server {:.2},{:.2})",
                server_tick,
                client_snapshot.position.x,
                client_snapshot.position.y,
                server.player.position.x,
                server.player.position.y
            );

            rollback_to_tick(
                server_tick,
                tick.current,
                &mut player.0,
                &snapshots,
                &commands,
            );
        }
    }
}

fn position_distance(a: Position, b: Position) -> f32 {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    (dx * dx + dy * dy).sqrt()
}

fn input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    socket: Res<ClientSocket>,
    mut history: ResMut<CommandHistory>,
    tick: Res<TickCounter>,
) {
    let mut dir = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    if dir != Vec2::ZERO {
        let dir = dir.normalize();
        let ticked = TickedCommand {
            tick: tick.current,
            command: MoveCommand {
                input: InputState {
                    move_x: dir.x,
                    move_y: dir.y,
                    cast: false,
                },
            },
        };

        let msg = ClientMessage::Command(ticked);
        let bytes = shared::serialize(&msg);
        let _ = socket.0.send_to(&bytes, "127.0.0.1:4000");

        // el cliente IGUAL guarda el comando para predicción
        history.commands.insert(tick.current, ticked.command);
    }
}
fn spawn_player_visual(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = meshes.add(Rectangle::new(30.0, 30.0));
    let material = materials.add(Color::srgb(0.3, 0.8, 0.4));

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(material),
        Transform::from_xyz(0.0, 0.0, 0.0),
        PlayerVisual,
        VisualSmoothing { target: Vec2::ZERO },
    ));
}

fn sync_player_visual(
    player: Res<PlayerResource>,
    mut query: Query<&mut VisualSmoothing, With<PlayerVisual>>,
) {
    let mut smoothing = query.single_mut().unwrap();
    smoothing.target = Vec2::new(player.0.position.x, player.0.position.y);
}

fn visual_smoothing_system(time: Res<Time>, mut query: Query<(&mut Transform, &VisualSmoothing)>) {
    let (mut transform, smoothing) = query.single_mut().unwrap();

    let current = transform.translation.truncate();
    let target = smoothing.target;

    let speed = 20.0; // visual only
    let new_pos = current.lerp(target, time.delta_secs() * speed);

    transform.translation.x = new_pos.x;
    transform.translation.y = new_pos.y;
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
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

fn rollback_test_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player: ResMut<PlayerResource>,
    snapshots: Res<SnapshotHistory>,
    commands: Res<CommandHistory>,
    tick: Res<TickCounter>,
) {
    if keyboard.just_pressed(KeyCode::KeyR) {
        let rollback_tick = tick.current.saturating_sub(30);

        info!("ROLLBACK: from tick {} to {}", tick.current, rollback_tick);

        rollback_to_tick(
            rollback_tick,
            tick.current,
            &mut player.0,
            &snapshots,
            &commands,
        );
    }
}

fn recv_snapshots(socket: Res<ClientSocket>, mut server: ResMut<ServerState>) {
    let mut buf = [0u8; 1024];

    while let Ok((len, _)) = socket.0.recv_from(&mut buf) {
        let msg: ServerMessage = shared::deserialize(&buf[..len]);

        if let ServerMessage::Snapshot { tick, player } = msg {
            server.tick = tick;
            server.player = player;
        }
    }
}
