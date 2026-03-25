use shared::{ids::PlayerId, input::ActionsFlags};

use crate::model::{ player::cast::{ CastMethod}, spells::elements::Element};



pub enum GameEvent{

    PlayerStartCast{
        owner:PlayerId,
        elements:[Element;3],
        method:CastMethod
    }



}

pub struct EventQueue{
    current: Vec<GameEvent>,
    next: Vec<GameEvent>
}

impl EventQueue {

    pub fn new() -> Self {
        Self {
            current: Vec::with_capacity(1024),
            next: Vec::with_capacity(1024),
        }
    }

    pub fn push(&mut self, event: GameEvent) {
        self.next.push(event);
    }

    pub fn drain_current(&mut self) -> std::vec::Drain<GameEvent> {
        self.current.drain(..)
    }

    pub fn swap(&mut self) {
        std::mem::swap(&mut self.current, &mut self.next);
        self.next.clear();
    }

    
}