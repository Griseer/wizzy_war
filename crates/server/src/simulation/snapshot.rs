//server/src/simulation/snapshot.rs

use crate::state::ServerState;
use net::server::message::{PlayerSnapshot, ServerMessage};
use shared::ids::PlayerId;
use shared::math::Vec3;
use shared::tick::InputTick;
use shared::{math::Vec2f, tick::ServerTick};

pub struct ServerSnapshot {
    pub tick: ServerTick,
    pub players: Vec<ServerPlayerSnapshot>,
}

pub struct ServerPlayerSnapshot {
    pub id: PlayerId,
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub aim: Vec2f,
    pub last_processed_input: InputTick,
}

pub fn snapshot_to_message(snapshot: &ServerSnapshot) -> ServerMessage {
    ServerMessage::Snapshot {
        server_tick: snapshot.tick,
        players: snapshot
            .players
            .iter()
            .map(|p| PlayerSnapshot {
                id: p.id,
                position: p.position,
                velocity: p.velocity,
                aim: p.aim,
                last_processed_input: p.last_processed_input,
            })
            .collect(),
    }
}

pub fn build_snapshot(state: &ServerState) -> ServerSnapshot {
    let mut players = Vec::with_capacity(state.players.len());

    for (id, player) in &state.players {
        players.push(ServerPlayerSnapshot {
            id: *id,
            position: player.state.position,
            velocity: player.state.velocity,
            aim: player.state.aim_dir,
            last_processed_input: player.last_processed_tick,
        });
    }

    ServerSnapshot {
        tick: state.tick,
        players,
    }
}
