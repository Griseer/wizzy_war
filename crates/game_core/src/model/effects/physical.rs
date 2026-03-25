use shared::math::Vec2f;

pub enum PhysicalEffectType {

    Pushed,
    Stumble,
    Airbone

}


pub struct PhysicalEffect{

    pub effect_type: PhysicalEffectType,
    pub horizontal_velocity: Vec2f,
    pub vertical_velocity:f32,
    pub end_tick:u32,


}