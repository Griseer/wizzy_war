use crate::math::{Vec2f, Vec3};
use crate::tick::Tick;

#[derive(Debug)]
pub struct InputFrame {
    pub tick: Tick,
    pub actions_flags: ActionsFlags,
    pub elements_flags: ElementsFlags,
    pub aim_target: Vec2f,
}

use bitflags::bitflags;

bitflags! {
    #[derive(Default,Debug)]
    pub struct ActionsFlags:u16{

        // cast
        const NORMAL_CAST = 1 << 0;
        const SELF_CAST = 1 << 1;
        
        const MOVE_TO = 1 << 2;
    }

    #[derive(Default, Clone, Debug, PartialEq, Copy)]
    pub struct ElementsFlags: u16{
        const WATER     = 1 << 1;
        const LIFE      = 1 << 2;
        const SHIELD    = 1 << 3;
        const COLD      = 1 << 4;
        const LIGHTNING = 1 << 5;
        const ARCANE    = 1 << 6;
        const EARTH     = 1 << 7;
        const FIRE      = 1 << 8;

    }
}
