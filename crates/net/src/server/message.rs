// --------------------
// ServerMessage
// --------------------

use crate::wire::*;
use shared::ids::PlayerId;
use shared::math::{Vec2f, Vec3};
use shared::tick::{Tick};

pub enum ServerMessage {
    Welcome {
        player_id: PlayerId,
        tick_rate: u64,
    },

    Snapshot {
        server_tick: Tick,
        players: Vec<PlayerSnapshot>,
    },
}

#[derive(Clone)]
pub struct PlayerSnapshot {
    pub id: PlayerId,
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub aim: Vec2f,
    pub elements:[u16;3],
    pub last_processed_input: Tick,
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
                write_u64(buf, *tick_rate);
            }

            ServerMessage::Snapshot {
                server_tick,
                players,
            } => {
                write_u8(buf, 1);
                write_u32(buf, server_tick.0);
                write_u16(buf, players.len() as u16);

                for p in players {
                    write_u64(buf, p.id.0);
                    write_f32(buf, p.position.x);
                    write_f32(buf, p.position.y);
                    write_f32(buf, p.velocity.x);
                    write_f32(buf, p.velocity.y);
                    write_f32(buf, p.aim.x);
                    write_f32(buf, p.aim.y);
                    for e in &p.elements {
                        write_u16(buf, *e);
                    }
                    write_u32(buf, p.last_processed_input.0);
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
                let tick_rate = read_u64(data, &mut c)?;

                Some(ServerMessage::Welcome {
                    player_id: id,
                    tick_rate,
                })
            }

            1 => {
                let server_tick = Tick(read_u32(data, &mut c)?);
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
                    let e1 = read_u16(data, &mut c)?;
                    let e2 = read_u16(data, &mut c)?;
                    let e3 = read_u16(data, &mut c)?;
                    let last_processed_input = Tick(read_u32(data, &mut c)?);

                    players.push(PlayerSnapshot {
                        id,
                        position: Vec2f { x: px, y: py },
                        velocity: Vec2f { x: vx, y: vy },
                        aim: Vec2f { x: fx, y: fy },
                        elements:[e1,e2,e3],
                        last_processed_input,
                    });
                }

                

                Some(ServerMessage::Snapshot {
                    server_tick,
                    players,
                })
            }

            _ => None,
        }
    }
}
