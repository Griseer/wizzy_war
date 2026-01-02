use crate::types::vec2::Vec2;

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub position: Vec2,
}

impl PlayerState {
    pub fn new() -> Self {
        PlayerState {
            position: Vec2::new(0.0, 0.0),
        }
    }
}
