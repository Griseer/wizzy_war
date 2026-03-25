//game_core/src/player.mod.rs

use shared::ids::PlayerId;
use shared::input::{ActionsFlags, InputFrame};
use shared::math::Vec2f;
use shared::tick::Tick;
use state::PlayerState;
use std::net::SocketAddr;

use intent::PlayerIntent;

use crate::model::effects::elemental::ElementalEffect;
use crate::model::effects::physical::PhysicalEffect;
use crate::model::physics::{Collider, PhysicBody};
use crate::model::player::build::PlayerBuild;
use crate::model::player::cast::PlayerCast;


pub mod intent;

pub mod state;

pub mod cast;

pub mod build;




pub struct Player {
    pub id: PlayerId,
    pub build: PlayerBuild,
    pub physic_body: PhysicBody,
    pub collider: Collider,
    pub intent: PlayerIntent,
    pub state: PlayerState,
    pub cast: PlayerCast,
    pub elemental_effect : Option<ElementalEffect>,
    pub physical_effect : Option<PhysicalEffect>,
    pub last_processed_tick: Tick,
}

impl Player {
    pub fn new(id: PlayerId) -> Self {
        Player {
            id,
            build: PlayerBuild::new(),
            physic_body: PhysicBody::default(),
            collider: Collider::cricle(5.0),
            intent: PlayerIntent::new(),
            state: PlayerState::new(),
            cast: PlayerCast::new(),
            elemental_effect: None,
            physical_effect: None,
            last_processed_tick: Tick(0),

        }
    }

    pub fn apply_input(&mut self, frame: &InputFrame) {
        // ignorar inputs viejos o repetidos
        if frame.tick <= self.last_processed_tick {
            return;
        }

        self.last_processed_tick = frame.tick;

        self.intent.update_intent(frame);
        
    }


    pub fn can_cast(&self) -> bool {

        
        true
    }


}


