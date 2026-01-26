//input/mod.rs

use bevy::prelude::*;
use shared::math::{Vec2f};
use shared::input::{Buttons, InputFrame};
use net::client::message::{ClientInputMessage, ClientMessage};

use crate::net::Network;
use crate::net::send::send_input as net_send_input;

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
    pub last_sent_tick: u64,
}

#[derive(Resource, Default)]
pub struct InputState {
    pub buttons: Buttons,
    pub aim_dir: Vec2f,
}

pub fn gather_input(
    time: Res<Time>,
    mouse: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    windows: Query<&Window>,
    mut state: ResMut<InputState>,
    mut buffer: ResMut<InputBuffer>,
) {
    let tick = (time.elapsed_secs_f64() * 60.0) as u64;

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

    // --- aim ---
    let Ok(window) = windows.single() else {
        return;
    };
    let cursor = window.cursor_position();
    let aim_dir = cursor
        .map(|p| Vec2f { x: p.x, y: p.y })
        .unwrap_or(state.aim_dir);

    if buttons.is_empty() 
        && (aim_dir.x.abs() - state.aim_dir.x).abs() == 0.0 
        && (aim_dir.y.abs() - state.aim_dir.y).abs() == 0.0
    {
        
        return;

    };

    state.aim_dir = aim_dir.clone();


    let input = InputFrame {
        tick,
        buttons,
        aim_dir,
        move_target: None, // se llena con click-move si lo implementas
    };



    buffer.inputs.push(input);
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
