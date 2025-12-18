// crates/client/src/main.rs
use bevy::prelude::*;
use std::{collections::HashMap, net::UdpSocket};

use game_core::PlayerId;
use net::{ClientMessage, ServerMessage};

#[derive(Resource, Default)]
struct RemoteWorld {
    players: HashMap<PlayerId, Vec2>,
}

#[derive(Resource)]
struct Network {
    socket: UdpSocket,
}

#[derive(Component)]
struct PlayerVisual {
    id: PlayerId,
}

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("127.0.0.1:4000")?;
    socket.set_nonblocking(true)?;

    // mandamos Join una sola vez
    let mut buf = Vec::new();
    ClientMessage::Join.encode(&mut buf);
    socket.send(&buf)?;

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Network { socket })
        .insert_resource(RemoteWorld::default())
        .add_systems(Startup, setup_camera)
        .add_systems(Update, receive_snapshots)
        .add_systems(Update, render_players)
        .add_systems(Update, send_input)
        .run();

    Ok(())
}

fn receive_snapshots(network: Res<Network>, mut world: ResMut<RemoteWorld>) {
    let mut buffer = [0u8; 2048];

    loop {
        match network.socket.recv(&mut buffer) {
            Ok(len) => {
                if let Some(ServerMessage::Snapshot { players }) =
                    ServerMessage::decode(&buffer[..len])
                {
                    world.players.clear();
                    for (id, state) in players {
                        world.players.insert(id, Vec2::new(state.x, state.y));
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                break;
            }
            Err(_) => break,
        }
    }
}

fn render_players(
    mut commands: Commands,
    world: Res<RemoteWorld>,
    mut query: Query<(Entity, &PlayerVisual, &mut Transform)>,
) {
    // actualizar existentes
    for (entity, visual, mut transform) in query.iter_mut() {
        if let Some(pos) = world.players.get(&visual.id) {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        } else {
            commands.entity(entity).despawn();
        }
    }

    // spawn faltantes
    for (id, pos) in world.players.iter() {
        if !query.iter().any(|(_, v, _)| v.id == *id) {
            let mut color_player = Color::WHITE;

            if id.0 == 2 {
                color_player = Color::BLACK;
            }

            commands.spawn((
                Sprite {
                    color: color_player,
                    custom_size: Some(Vec2::splat(20.0)),
                    ..default()
                },
                Transform::from_xyz(pos.x, pos.y, 0.0),
                GlobalTransform::default(),
                PlayerVisual { id: *id },
            ));
        }
    }
}
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn send_input(keyboard: Res<ButtonInput<KeyCode>>, network: Res<Network>) {
    let mut dx = 0.0;
    let mut dy = 0.0;

    let speed = 2.0;

    if keyboard.pressed(KeyCode::KeyW) {
        dy += speed;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        dy -= speed;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        dx -= speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dx += speed;
    }

    if dx != 0.0 || dy != 0.0 {
        let mut buffer = Vec::new();
        ClientMessage::Move { dx, dy }.encode(&mut buffer);
        let _ = network.socket.send(&buffer);
    }
}
