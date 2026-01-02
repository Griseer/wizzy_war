
use std::net::{SocketAddr};
use std::collections::{HashMap};
use game_core::PlayerId;
use net::client::message::ClientInputMessage;


pub struct ServerState {
    pub tick: u64,
    pub players: HashMap<PlayerId, Player>,
    pub input_buffers: HashMap<PlayerId, InputBuffer>,
}


impl ServerState {
    pub fn new() -> Self { ... }

    pub fn add_player(&mut self, addr: SocketAddr) -> PlayerId { ... }

    pub fn handle_input(&mut self, addr: SocketAddr, msg: ClientInputMessage) { ... }
}
