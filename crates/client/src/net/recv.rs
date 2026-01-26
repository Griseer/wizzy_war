// client/src/net/recv.rs

use bevy::prelude::*;
use shared::ids::PlayerId;
use net::server::message::ServerMessage;

use super::Network;
//use crate::world::RemoteWorld;

pub fn receive_snapshots(network: Res<Network> /* mut world: ResMut<RemoteWorld> */) {
    let mut buffer = [0u8; 2048];

    loop {
        match network.socket.recv(&mut buffer) {
            Ok(len) => match ServerMessage::decode(&buffer[..len]) {
                Some(ServerMessage::Snapshot { players, .. }) => {
                    //world.players.clear();
                    //for (id, state) in players {
                    //    world.players.insert(id, Vec2::new(state.x, state.y));
                    //}
                }

                Some(ServerMessage::Welcome {
                    player_id,
                    tick_rate,
                }) => {
                    println!("welcome player id: {:?}", player_id)
                }

                None => {
                    break;
                }
            },
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
            Err(_) => break,
        }
    }
}
