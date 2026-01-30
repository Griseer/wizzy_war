use crate::math::Vec2f;
use crate::tick::InputTick;


#[derive(Debug)]
pub struct InputFrame {
    pub tick: InputTick,
    pub buttons: Buttons,
    pub aim_dir: Vec2f,
    pub move_target: Option<Vec2f>,

}



use bitflags::bitflags;


bitflags! {
    #[derive(Default,Debug)]
    pub struct Buttons:u16{
        // elements
        const WATER     = 1 << 0;
        const LIFE      = 1 << 1;
        const SHIELD    = 1 << 2;
        const COLD      = 1 << 3;
        const LIGHTNING = 1 << 4;
        const ARCANE    = 1 << 5;
        const EARTH     = 1 << 6;
        const FIRE      = 1 << 7;
        // cast
        const NORMAL_CAST = 1 << 8;
        const SELF_CAST = 1 << 9;
        // Move
        const MOVE_CLICK = 1 << 10;
    }
}

