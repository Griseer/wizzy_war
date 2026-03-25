use shared::math::Vec2f;

use crate::model::spells::{Spell, SpellType, elements::Element, elements::ElementKind};



pub enum CastMethod{
    NormalCast,
    SelfCast,
}


pub enum CastAnimation{
    Spray,
    Beam,
    AoE,
    Earthquake,
    Channel,
    Charge,
    

}



pub struct Cast {
    
    pub animation: CastAnimation,
    pub hold: Option<CastMethod>,
    pub elements: [Element; 3],
    pub start_tick:u32,
    
}


pub struct PlayerCast {
    pub elements: [Element; 3],
    pub cast: Option<Cast>,
}

impl PlayerCast {
    pub fn new() -> Self {
        Self {
            elements: [Element::new(ElementKind::Zero); 3],
            cast: None,
        }
    }

    pub fn elements_to_bits(&self) -> [u16; 3] {
        [
            self.elements[0].data.element_bits,
            self.elements[1].data.element_bits,
            self.elements[2].data.element_bits,
        ]
    }
}
