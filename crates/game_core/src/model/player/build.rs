

pub struct PlayerBuild {
    

    
    pub max_health: f32,
    pub speed: f32,


}

impl PlayerBuild {
    pub fn new() -> Self {
        Self {
            speed:6.0,
            max_health: 1500.0,
        }
    }
}
