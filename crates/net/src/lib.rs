// NET
use game_core::PlayerId;

#[derive(Debug)]
pub enum ClientMessage {
    Join,
}

#[derive(Debug)]
pub enum ServerMessage {
    Welcome { player_id: PlayerId },
}

impl ClientMessage {
    pub fn encode(&self, buffer: &mut Vec<u8>) {
        match self {
            ClientMessage::Join => buffer.push(0),
        }
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        match data.first()? {
            0 => Some(ClientMessage::Join),
            _ => None,
        }
    }
}

impl ServerMessage {
    pub fn encode(&self, buffer: &mut Vec<u8>) {
        match self {
            ServerMessage::Welcome { player_id } => {
                buffer.push(1);
                buffer.extend_from_slice(&player_id.0.to_be_bytes());
            }
        }
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        match data.first()? {
            1 => {
                let id = u32::from_be_bytes(data.get(1..5)?.try_into().ok()?);
                Some(ServerMessage::Welcome {
                    player_id: PlayerId(id),
                })
            }
            _ => None,
        }
    }
}
