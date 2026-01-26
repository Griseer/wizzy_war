// server/src/net/send.rs

use ::net::server::message::ServerMessage;
use shared::ids::PlayerId;
use std::net::{SocketAddr, UdpSocket};

pub fn send_welcome(socket: &UdpSocket, addr: SocketAddr, player_id: PlayerId) {
    let mut buf = Vec::with_capacity(16);

    let msg = ServerMessage::Welcome {
        player_id,
        tick_rate: 0_u16,
    };
    msg.encode(&mut buf);

    let _ = socket.send_to(&buf, addr);
}

//pub fn broadcast_snapshot(socket: &UdpSocket, state: &ServerState);
