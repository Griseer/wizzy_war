//client/src/net/mod.rs

use ::net::client::message::ClientMessage;
use bevy::prelude::*;
use std::net::UdpSocket;

pub mod recv;
pub mod send;

pub struct NetPlugin {
    pub socket: UdpSocket,
}

impl NetPlugin {
    pub fn new(socket: UdpSocket) -> Self {
        Self { socket }
    }
}

impl Plugin for NetPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Network {
            socket: self.socket.try_clone().unwrap(),
        })
        .add_systems(Update, recv::receive_snapshots);
    }
}

#[derive(Resource)]
pub struct Network {
    pub socket: UdpSocket,
}
