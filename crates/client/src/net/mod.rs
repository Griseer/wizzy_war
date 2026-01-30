//client/src/net/mod.rs

use ::net::client::message::ClientMessage;
use bevy::prelude::*;
use std::net::UdpSocket;
use net::server::message::ServerMessage;
use crate::{net::recv::recv_network_system, state::ClientState};

pub mod recv;
pub mod send;

#[derive(Resource)]
pub struct Network {
    pub socket: UdpSocket,
    pub incoming: Vec<ServerMessage>,
}
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
            incoming: Vec::new(),
        })
        .add_systems(Update,( recv_network_system ,apply_server_messages_system));
    }
}


pub fn apply_server_messages_system(
    mut network: ResMut<Network>,
    mut state: ResMut<ClientState>,
) {
    for msg in network.incoming.drain(..) {
        match msg {
            ServerMessage::Welcome { player_id, tick_rate } => {
                println!("Welcome! id={:?} tick_rate={}", player_id, tick_rate);
            }

            ServerMessage::Snapshot { players: snaps, .. } => {
                for snap in snaps {
                    state.apply_snapshot(snap);
                }
            }
        }
    }
}