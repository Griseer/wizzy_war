//CLIENT

use bevy::{input::ButtonInput, prelude::*};
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

#[derive(Component)]
struct PlayerVisual;

fn main() {
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
        .add_systems(FixedUpdate, simulation_step)
        .add_systems(Update, input_system)
        .add_systems(Update, sync_player_visual)
        .add_systems(Startup, (setup_camera, spawn_player_visual))
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

fn input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
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

        history.commands.insert(
            tick.current,
            MoveCommand {
                dir_x: dir.x,
                dir_y: dir.y,
            },
        );
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
    ));
}

fn sync_player_visual(
    player: Res<PlayerResource>,
    mut query: Query<&mut Transform, With<PlayerVisual>>,
) {
    let mut transform = query.single_mut().unwrap();
    transform.translation.x = player.0.position.x;
    transform.translation.y = player.0.position.y;
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}
