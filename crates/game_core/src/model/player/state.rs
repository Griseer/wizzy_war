use std::prelude::*;


use shared::input::{ActionsFlags, ElementsFlags, InputFrame};
use shared::math::{Vec2f, Vec3};





pub struct PlayerState {
    pub aim: Vec2f,
    pub health: f32,

    pub target: Option<Vec2f>
    // pub shield_one: Option<Element>,
    // pub shield_two: Option<Element>,

}

impl PlayerState {
    pub fn new() -> Self {
        Self {
            aim: Vec2f::ZERO,
            health: 1500.0,
            target:None

        }
    }



    // pub fn set_aim(&mut self, frame: &InputFrame) {
    //     self.aim_dir = frame.aim_target;
    // }

    

    // pub fn set_action(&mut self, frame: &InputFrame) {
    //     if self.cooldown > 0.0 {
    //         return;
    //     }

    //     let normal_cast = frame.actions.contains(Actions::NORMAL_CAST);

    //     let self_cast = frame.actions.contains(Actions::SELF_CAST);

    //     if !normal_cast && !self_cast {
    //         self.cast = Cast::None;
    //         return;
    //     }

    //     match &mut self.cast {
    //         Cast::None => {
    //             if self_cast {
    //                 self.cast = Cast::SelfCast(0.0)
    //             }
    //             if normal_cast {
    //                 self.cast = Cast::NormalCast(0.0)
    //             }
    //         }

    //         Cast::SelfCast(_) => {
    //             if normal_cast {
    //                 self.cast = Cast::NormalCast(0.0)
    //             }
    //         }

    //         Cast::NormalCast(_) => {
    //             if self_cast {
    //                 self.cast = Cast::SelfCast(0.0)
    //             }
    //         }
    //     }
    // }
}
