#[derive(Clone, Copy, Debug)]
pub struct PlayerId(pub u32);

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Debug)]
pub struct Player {
    pub id: PlayerId,
    pub position: Position,
}

#[derive(Clone, Copy, Debug)]
pub struct MoveCommand {
    pub dir_x: f32,
    pub dir_y: f32,
}

#[derive(Clone, Copy, Debug)]
pub struct TickedCommand {
    pub tick: u64,
    pub command: MoveCommand,
}

use shared::TICK_RATE;

pub const PLAYER_SPEED: f32 = 20.0;

pub fn simulate_player(player: &mut Player, cmd: Option<MoveCommand>) {
    if let Some(cmd) = cmd {
        let dx = cmd.dir_x * PLAYER_SPEED / TICK_RATE;
        let dy = cmd.dir_y * PLAYER_SPEED / TICK_RATE;

        player.position.x += dx;
        player.position.y += dy;
    }
}
