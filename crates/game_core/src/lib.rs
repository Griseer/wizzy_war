// game_core

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct InputState {
    pub move_x: f32,
    pub move_y: f32,
    pub cast: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServerSnapshot {
    pub tick: u64,
    pub player: Player,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct PlayerId(pub u32);

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: PlayerId,
    pub position: Position,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct MoveCommand {
    pub input: InputState,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct TickedCommand {
    pub tick: u64,
    pub command: MoveCommand,
}

use shared::TICK_RATE;

pub const PLAYER_SPEED: f32 = 50.0;

pub fn simulate_player(player: &mut Player, cmd: Option<MoveCommand>) {
    if let Some(cmd) = cmd {
        let dx = cmd.input.move_x * PLAYER_SPEED / TICK_RATE;
        let dy = cmd.input.move_y * PLAYER_SPEED / TICK_RATE;

        player.position.x += dx;
        player.position.y += dy;
    }
}
