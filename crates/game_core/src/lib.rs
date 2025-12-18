// game_core

pub mod world;

pub use world::{PlayerState, WorldState};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u32);
