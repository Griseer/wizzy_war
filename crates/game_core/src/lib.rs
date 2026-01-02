//// game_core
//
//pub mod world;
//
//pub mod player;
//
//pub mod spells;
//
pub mod types;
//
////pub use world::{PlayerState, WorldState};
//
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlayerId(pub u64);
