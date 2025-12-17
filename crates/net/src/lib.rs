use game_core::{Player, TickedCommand};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientMessage {
    Command(TickedCommand),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerMessage {
    Snapshot { tick: u64, player: Player },
}
