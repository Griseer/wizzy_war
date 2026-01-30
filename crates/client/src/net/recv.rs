// client/src/net/recv.rs

use bevy::prelude::*;
use shared::ids::PlayerId;
use net::server::message::ServerMessage;
use std::net::UdpSocket;

use super::Network;
//use crate::world::RemoteWorld;

pub fn recv_network_system(
    mut network: ResMut<Network>,
) {
    let msgs = recv_messages(&network.socket);
    network.incoming.extend(msgs);
}




pub fn recv_messages(socket: &UdpSocket) -> Vec<ServerMessage> {
    let mut messages = Vec::new();
    let mut buffer = [0u8; 2048];

    loop {
        match socket.recv(&mut buffer) {
            Ok(len) => {
                if let Some(msg) = ServerMessage::decode(&buffer[..len]) {
                    messages.push(msg);
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(_) => break,
        }
    }

    messages
}