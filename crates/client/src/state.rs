
// crates/client/src/state.rs

use bevy::prelude::*;
use shared::ids::PlayerId;

use std::collections::HashMap;
use shared::math::Vec2f;

use net::server::message::PlayerSnapshot;


pub struct StatePlugin;

impl Plugin for StatePlugin{
    fn build(&self, app: &mut App) {
        app.init_resource::<ClientState>();
        
    }
}

pub struct ClientPlayer {
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub aim: Vec2f,
}




#[derive(Resource, Default)]
pub struct ClientState {
    pub local_player_id: PlayerId,

    // tick del server que estamos aceptando
    pub last_server_tick: u64,

    // estado autoritativo (último snapshot)
    pub server_players: HashMap<PlayerId, PlayerSnapshot>,

    // estado predicho local
    pub client_players: HashMap<PlayerId, ClientPlayer>,

    // inputs enviados pero no confirmados
    //pub pending_inputs: Vec<PlayerInput>,

    pub tick_rate: u64,
}

impl ClientState {
    pub fn new() -> Self {
        Self {
            local_player_id: PlayerId(0), // temporal
            last_server_tick: 0,
            server_players: HashMap::new(),
            client_players: HashMap::new(),
            tick_rate: 0,
        }
    }
    
    pub fn apply_snapshot(&mut self, snap: PlayerSnapshot) {
        
        // por ahora: espejo directo (sin prediction)
        self.client_players.insert(
            snap.id,
            ClientPlayer {
                position: snap.position,
                velocity: snap.velocity,
                aim: snap.aim,
            },
        );
        self.server_players.insert(snap.id, snap.clone());
    }
}

