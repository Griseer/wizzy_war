//server/src/simulation/snapshot.rs

use crate::state::ServerState;
use net::server::message::{PlayerSnapshot, ServerMessage};
use shared::ids::PlayerId;
use shared::math::Vec3;
use shared::tick::Tick;
use shared::{math::Vec2f};

pub struct ServerSnapshot {
    pub tick: Tick,
    pub players: Vec<ServerPlayerSnapshot>,
}

pub struct ServerPlayerSnapshot {
    pub id: PlayerId,
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub aim: Vec2f,
    pub elements: [u16;3],
    pub last_processed_input: Tick,
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
                elements: p.elements,
                last_processed_input: p.last_processed_input,
            })
            .collect(),
    }
}

pub fn build_snapshot(state: &ServerState) -> ServerSnapshot {
    let mut players = Vec::with_capacity(state.world.players.len());

    for (id, player) in &state.world.players {
        players.push(ServerPlayerSnapshot {
            id: *id,
            position: player.physic_body.position,
            velocity: player.physic_body.velocity,
            aim: player.state.aim,
            elements: player.cast.elements_to_bits(),
            last_processed_input: player.last_processed_tick,
        });
    }

    ServerSnapshot {
        tick: state.tick,
        players,
    }
}
