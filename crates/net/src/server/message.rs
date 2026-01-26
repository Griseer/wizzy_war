// --------------------
// ServerMessage
// --------------------

use crate::wire::*;
use shared::ids::PlayerId;
use shared::math::{Vec2f};

pub enum ServerMessage {
    Welcome {
        player_id: PlayerId,
        tick_rate: u16,
    },

    Snapshot {
        server_tick: u64,
        last_processed_input: u64,
        players: Vec<PlayerSnapshot>,
    },
}

pub struct PlayerSnapshot {
    pub id: PlayerId,
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub facing: Vec2f,
}

impl ServerMessage {
    pub fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            ServerMessage::Welcome {
                player_id,
                tick_rate,
            } => {
                write_u8(buf, 0);
                write_u64(buf, player_id.0);
                write_u16(buf, *tick_rate);
            }

            ServerMessage::Snapshot {
                server_tick,
                last_processed_input,
                players,
            } => {
                write_u8(buf, 1);
                write_u64(buf, *server_tick);
                write_u64(buf, *last_processed_input);
                write_u16(buf, players.len() as u16);

                for p in players {
                    write_u64(buf, p.id.0);
                    write_f32(buf, p.position.x);
                    write_f32(buf, p.position.y);
                    write_f32(buf, p.velocity.x);
                    write_f32(buf, p.velocity.y);
                    write_f32(buf, p.facing.x);
                    write_f32(buf, p.facing.y);
                }
            }
        }
    }
}

impl ServerMessage {
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut c = 0;

        match read_u8(data, &mut c)? {
            0 => {
                let id = PlayerId(read_u64(data, &mut c)?);
                let tick_rate = read_u16(data, &mut c)?;

                Some(ServerMessage::Welcome {
                    player_id: id,
                    tick_rate,
                })
            }

            1 => {
                let server_tick = read_u64(data, &mut c)?;
                let last_processed_input = read_u64(data, &mut c)?;
                let count = read_u16(data, &mut c)? as usize;

                if count > 128 {
                    return None;
                }

                let mut players = Vec::with_capacity(count);

                for _ in 0..count {
                    let id = PlayerId(read_u64(data, &mut c)?);

                    let px = read_f32(data, &mut c)?;
                    let py = read_f32(data, &mut c)?;
                    let vx = read_f32(data, &mut c)?;
                    let vy = read_f32(data, &mut c)?;
                    let fx = read_f32(data, &mut c)?;
                    let fy = read_f32(data, &mut c)?;

                    players.push(PlayerSnapshot {
                        id,
                        position: Vec2f { x: px, y: py },
                        velocity: Vec2f { x: vx, y: vy },
                        facing: Vec2f { x: fx, y: fy },
                    });
                }

                Some(ServerMessage::Snapshot {
                    server_tick,
                    last_processed_input,
                    players,
                })
            }

            _ => None,
        }
    }
}
