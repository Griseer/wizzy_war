pub mod elements;


use shared::{ids::PlayerId, math::Vec2f};
use slotmap::{SlotMap, new_key_type};



use crate::model::{
    spells::elements::Element,
    world::{World},
    physics::Collider
};



new_key_type! {
    pub struct SpellId;

}




pub enum SpellType {
    
    //Shield(Shield),
    Projectile(Projectile),
    //Lightning(Lightning),
    //Beam(Beam),
    //Sparay(Sparay),
    //Mine(Mine),
    //Area(Area),
}

pub struct Spell {
    owner: PlayerId,
    spellkind: SpellType,
}

impl Spell {

}

pub struct Projectile {


    
    collider: Collider,
    velocity: Vec2f,
    position: Vec2f,
    lifetime: f32,


    //mask_layer: u32,
}

impl Projectile {

}

pub struct Shield {
    collider: Collider,
    position: Vec2f,
    lifetime: f32,
    mask_layer: u32,
}

impl Shield {
    fn update(&self, dt: f32) {}
}

pub struct Lightning {
    collider: Collider,
    position: Vec2f,
    lifetime: f32,
    mask_layer: u32,
}

impl Lightning {
    fn update(&self, dt: f32) {}
}

pub struct Beam {
    collider: Collider,
    position: Vec2f,
    lifetime: f32,
    mask_layer: u32,
}

impl Beam {
    fn update(&self, dt: f32) {}
}

pub struct Sparay {
    collider: Collider,
    position: Vec2f,
    lifetime: f32,
    mask_layer: u32,
}

impl Sparay {
    fn update(&self, dt: f32) {}
}


pub struct Mine {
    collider: Collider,
    position: Vec2f,
    lifetime: f32,
    mask_layer: u32,
}

impl Mine {
    fn update(&self, dt: f32) {}
}


pub struct Area {
    collider: Collider,
    position: Vec2f,
    lifetime: f32,
    mask_layer: u32,
}

impl Area {
    fn update(&self, dt: f32) {}
}
