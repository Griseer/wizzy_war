// --------------------
// ClientMessage
// --------------------

use crate::wire::*;
use shared::input::{ActionsFlags, ElementsFlags, InputFrame};
use shared::math::{Vec2f, Vec2i};
use shared::tick::Tick;

#[derive(Debug)]
pub struct ClientInputMessage {
    pub last_ack_tick: Tick,
    pub inputs: Vec<InputFrame>,
}

#[derive(Debug)]
pub enum ClientMessage {
    Join,
    Input(ClientInputMessage),
}

impl ClientMessage {
    pub fn encode(&self, buf: &mut Vec<u8>) {
        match self {
            ClientMessage::Join => {
                write_u8(buf, 0);
            }
            ClientMessage::Input(input) => {
                write_u8(buf, 1);
                input.encode(buf);
            }
        }
    }

    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut cursor = 0;

        let tag = read_u8(data, &mut cursor)?;

        match tag {
            0 => {
                // Join no tiene payload
                Some(ClientMessage::Join)
            }

            1 => {
                let input = ClientInputMessage::decode(&data[cursor..])?;
                Some(ClientMessage::Input(input))
            }

            _ => None,
        }
    }
}

impl ClientInputMessage {
    pub fn encode(&self, buf: &mut Vec<u8>) {
        write_u32(buf, self.last_ack_tick.0);
        write_u8(buf, self.inputs.len() as u8);

        for input in &self.inputs {
            write_u32(buf, input.tick.0);
            write_u16(buf, input.actions_flags.bits());
            write_u16(buf, input.elements_flags.bits());
            write_f32(buf, input.aim_target.x);
            write_f32(buf, input.aim_target.y);
        }
    }
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut c = 0;

        let last_ack_tick = Tick(read_u32(data, &mut c)?);
        let count = read_u8(data, &mut c)? as usize;

        if count == 0 || count > 32 {
            return None;
        }

        let mut inputs = Vec::with_capacity(count);

        for _ in 0..count {
            let tick = Tick(read_u32(data, &mut c)?);
            let actions_bits = read_u16(data, &mut c)?;
            let elements_bits = read_u16(data, &mut c)?;
            let actions_flags = ActionsFlags::from_bits_truncate(actions_bits);
            let elements_flags = ElementsFlags::from_bits_truncate(elements_bits);
            let x = read_f32(data, &mut c)?;
            let y = read_f32(data, &mut c)?;

            let aim_target = Vec2f { x, y };

            inputs.push(InputFrame {
                tick,
                actions_flags,
                elements_flags,
                aim_target,
            });
        }

        Some(ClientInputMessage {
            last_ack_tick,
            inputs,
        })
    }
}
