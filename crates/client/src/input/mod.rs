//input/mod.rs

use bevy::prelude::*;
use net::client::message::{ClientInputMessage, ClientMessage};
use shared::input::{Buttons, InputFrame};
use shared::math::Vec2f;
use shared::tick::InputTick;

use crate::net::Network;
use crate::net::send::send_input as net_send_input;
use crate::render::MyCamera;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputBuffer>();
        app.init_resource::<InputState>();
        app.add_systems(Update, gather_input);
        app.add_systems(Update, send_input);
    }
}

#[derive(Resource, Default)]
pub struct InputBuffer {
    pub inputs: Vec<InputFrame>,
    pub next_tick: InputTick,
    pub last_sent_tick: InputTick,
}

#[derive(Resource, Default)]
pub struct InputState {
    pub buttons: Buttons,
    pub aim_target: Vec2f,
}

pub fn gather_input(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
    mut state: ResMut<InputState>,
    mut buffer: ResMut<InputBuffer>,
) {
    let tick = buffer.next_tick;
    buffer.next_tick = InputTick(buffer.next_tick.0 + 1);

    let mut buttons = Buttons::empty();

    // --- elementos ---
    if keyboard.pressed(KeyCode::KeyQ) {
        buttons |= Buttons::WATER;
    }
    if keyboard.pressed(KeyCode::KeyW) {
        buttons |= Buttons::LIFE;
    }
    if keyboard.pressed(KeyCode::KeyE) {
        buttons |= Buttons::SHIELD;
    }
    if keyboard.pressed(KeyCode::KeyR) {
        buttons |= Buttons::COLD;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        buttons |= Buttons::LIGHTNING;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        buttons |= Buttons::ARCANE;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        buttons |= Buttons::EARTH;
    }
    if keyboard.pressed(KeyCode::KeyF) {
        buttons |= Buttons::FIRE;
    }
    // --- cast ---
    if mouse.just_pressed(MouseButton::Right) {
        buttons |= Buttons::NORMAL_CAST;
    }
    if mouse.just_pressed(MouseButton::Middle) {
        buttons |= Buttons::SELF_CAST;
    }

    // ---- Move ----
    if mouse.just_pressed(MouseButton::Left) {
        buttons |= Buttons::MOVE_TO;
    }

    // --- aim ---

    let aim_target = get_aim(windows, camera_q);

    if buttons.is_empty() && aim_target == state.aim_target {
        return;
    }

    let input = InputFrame {
        tick,
        buttons,
        aim_target,
    };

    buffer.inputs.push(input);
}

fn get_aim(
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera3d>>,
) -> Vec2f {
    let mut aim_target = Vec2f::ZERO;

    let Ok(window) = windows.single() else {
        return aim_target;
    };
    let Some(cursor) = window.cursor_position() else {
        return aim_target;
    };

    let Ok((camera, camera_transform)) = camera_q.single() else {
        return aim_target;
    };

    let Ok(ray) = camera.viewport_to_world(camera_transform, cursor) else {
        return aim_target;
    };

    let origin = ray.origin;
    let direction = ray.direction;
    let plane_y = 0.0;

    // Evitar división por cero
    if direction.y.abs() > 0.0001 {
        let t = (plane_y - origin.y) / direction.y;

        if t > 0.0 {
            let hit_point = origin + direction * t;
            aim_target = Vec2f {
                x: hit_point.x,
                y: hit_point.z,
            };
        }
    }
    aim_target
}

pub fn send_input(net: Res<Network>, mut buffer: ResMut<InputBuffer>) {
    if buffer.inputs.is_empty() {
        return;
    }

    let msg = ClientMessage::Input(ClientInputMessage {
        last_ack_tick: buffer.last_sent_tick,
        inputs: buffer.inputs.drain(..).collect(),
    });

    net_send_input(&net.socket, &msg);

    if let ClientMessage::Input(m) = msg {
        if let Some(last) = m.inputs.last() {
            buffer.last_sent_tick = last.tick;
        }
    }
}
