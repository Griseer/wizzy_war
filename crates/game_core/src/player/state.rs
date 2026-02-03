use std::vec;

use crate::spells::elements::Element;
use shared::math::{Vec2f, Vec3};

#[derive(Clone, Debug)]
pub struct PlayerState {
    pub position: Vec2f,
    pub velocity: Vec2f,
    pub aim_dir: Vec2f,
    pub hp: i32,
    pub move_speed: f32,
    pub move_target: Option<Vec2f>,
    // Magicka
    pub element_buffer: Vec<Element>,
    pub is_casting: bool,
}

impl PlayerState {
    pub fn new() -> Self {
        PlayerState {
            position: Vec2f::ZERO,
            velocity: Vec2f { x: 0.0, y: 0.0 },
            aim_dir: Vec2f::ZERO,
            hp: 1500,
            move_speed: 6.0,
            move_target: None,
            element_buffer: Vec::new(),
            is_casting: false,
        }
    }
}
