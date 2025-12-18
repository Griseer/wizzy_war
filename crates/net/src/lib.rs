// NET
use game_core::{PlayerId, PlayerState};

#[derive(Debug)]
pub enum ClientMessage {
    Join,
    Move { dx: f32, dy: f32 },
}

#[derive(Debug)]
pub enum ServerMessage {
    Welcome {
        player_id: PlayerId,
    },
    Snapshot {
        players: Vec<(PlayerId, PlayerState)>,
    },
}

// --------------------
// ClientMessage
// --------------------

impl ClientMessage {
    pub fn encode(&self, buffer: &mut Vec<u8>) {
        match self {
            ClientMessage::Join => buffer.push(0),
            ClientMessage::Move { dx, dy } => {
                buffer.push(1);
                buffer.extend_from_slice(&dx.to_be_bytes());
                buffer.extend_from_slice(&dy.to_be_bytes());
            }
        }
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        match data.first()? {
            0 => Some(ClientMessage::Join),
            1 => {
                let dx = f32::from_be_bytes(data.get(1..5)?.try_into().ok()?);
                let dy = f32::from_be_bytes(data.get(5..9)?.try_into().ok()?);
                Some(ClientMessage::Move { dx, dy })
            }
            _ => None,
        }
    }
}

// --------------------
// ServerMessage
// --------------------

impl ServerMessage {
    pub fn encode(&self, buffer: &mut Vec<u8>) {
        match self {
            ServerMessage::Welcome { player_id } => {
                buffer.push(1);
                buffer.extend_from_slice(&player_id.0.to_be_bytes());
            }

            ServerMessage::Snapshot { players } => {
                buffer.push(2);
                buffer.extend_from_slice(&(players.len() as u32).to_be_bytes());

                for (id, state) in players {
                    buffer.extend_from_slice(&id.0.to_be_bytes());
                    buffer.extend_from_slice(&state.x.to_be_bytes());
                    buffer.extend_from_slice(&state.y.to_be_bytes());
                }
            }
        }
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        match data.first()? {
            // Welcome
            1 => {
                let id = u32::from_be_bytes(data.get(1..5)?.try_into().ok()?);
                Some(ServerMessage::Welcome {
                    player_id: PlayerId(id),
                })
            }

            // Snapshot
            2 => {
                let mut offset = 1;

                let count =
                    u32::from_be_bytes(data.get(offset..offset + 4)?.try_into().ok()?) as usize;
                offset += 4;

                let mut players = Vec::with_capacity(count);

                for _ in 0..count {
                    let id = u32::from_be_bytes(data.get(offset..offset + 4)?.try_into().ok()?);
                    offset += 4;

                    let x = f32::from_be_bytes(data.get(offset..offset + 4)?.try_into().ok()?);
                    offset += 4;

                    let y = f32::from_be_bytes(data.get(offset..offset + 4)?.try_into().ok()?);
                    offset += 4;

                    players.push((PlayerId(id), PlayerState { x, y }));
                }

                Some(ServerMessage::Snapshot { players })
            }

            _ => None,
        }
    }
}
