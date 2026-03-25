use std::sync::{BarrierWaitResult, atomic::ATOMIC_USIZE_INIT};

use shared::input::ElementsFlags;

use crate::model::{
    player::Player,
    spells::elements::{self, Element, ElementKind},
    world::World,
};

pub fn run(world: &mut World) {
    for player in world.players.values_mut() {

    }
}




