// --------------------
// ClientMessage
// --------------------

use crate::wire::*;
use shared::input::{Buttons, InputFrame};
use shared::math::{Vec2f, Vec2i};
use shared::tick::InputTick;

#[derive(Debug)]
pub struct ClientInputMessage {
    pub last_ack_tick: InputTick,
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
        write_u64(buf, self.last_ack_tick.0);
        write_u8(buf, self.inputs.len() as u8);

        for input in &self.inputs {
            write_u64(buf, input.tick.0);
            write_u16(buf, input.buttons.bits());
            write_f32(buf, input.aim_dir.x);
            write_f32(buf, input.aim_dir.y);
            if input.buttons.contains(Buttons::MOVE_CLICK) {
                let target = input.move_target.expect("MOVE_CLICK without target");
                write_f32(buf, target.x);
                write_f32(buf, target.y);
            }
        }
    }
    pub fn decode(data: &[u8]) -> Option<Self> {
        let mut c = 0;

        let last_ack_tick = InputTick(read_u64(data, &mut c)?);
        let count = read_u8(data, &mut c)? as usize;

        if count == 0 || count > 32 {
            return None;
        }

        let mut inputs = Vec::with_capacity(count);

        for _ in 0..count {
            let tick = InputTick(read_u64(data, &mut c)?);
            let bits = read_u16(data, &mut c)?;
            let buttons = Buttons::from_bits_truncate(bits);

            let x = read_f32(data, &mut c)?;
            let y = read_f32(data, &mut c)?;

            let move_target = if buttons.contains(Buttons::MOVE_CLICK) {
                let x = read_f32(data, &mut c)?;
                let y = read_f32(data, &mut c)?;
                Some(Vec2f { x, y })
            } else {
                None
            };

            inputs.push(InputFrame {
                tick,
                buttons,
                aim_dir: Vec2f { x, y },
                move_target,
            });
        }

        Some(ClientInputMessage {
            last_ack_tick,
            inputs,
        })
    }
}
