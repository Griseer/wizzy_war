// server/src/net/send.rs

use ::net::server::message::ServerMessage;
use shared::ids::PlayerId;
use std::net::{SocketAddr, UdpSocket};
use crate::{simulation::snapshot::snapshot_to_message, state::ServerState};

pub fn send_welcome(socket: &UdpSocket, addr: SocketAddr, player_id: PlayerId) {
    let mut buf = Vec::with_capacity(16);

    let msg = ServerMessage::Welcome {
        player_id,
        tick_rate: 0_u64,
    };
    msg.encode(&mut buf);

    let _ = socket.send_to(&buf, addr);
}

pub fn broadcast_snapshot(socket: &UdpSocket, state:&ServerState){

    let msg: ServerMessage = snapshot_to_message(&state.last_snapshot);

    let mut buf = Vec::with_capacity(1024);
    msg.encode(&mut buf);

    for player_addr in state.addr_players.keys() {
        let _ = socket.send_to(&buf, player_addr);
    }
}
