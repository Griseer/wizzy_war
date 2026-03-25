use shared::{
    input::{ActionsFlags, ElementsFlags, InputFrame},
    math::Vec2f,
};

pub struct PlayerIntent {
    pub target: Option<Vec2f>,
    pub loock_at: Vec2f,
    pub element_inputs: ElementsFlags,
    pub normal_cast: bool,
    pub self_cast: bool,

}

impl PlayerIntent {
    pub fn new() -> Self {
        PlayerIntent {
            target: None,
            loock_at: Vec2f::ZERO,
            element_inputs: ElementsFlags::empty(),
            normal_cast: false,
            self_cast: false,
            
        }
    }

    pub fn update_intent(&mut self, frame: &InputFrame){
        self.target = if frame.actions_flags.intersects(ActionsFlags::MOVE_TO) {
            Some(frame.aim_target)
        } else {
            None
        };

        self.element_inputs = frame.elements_flags.clone();

        self.loock_at = frame.aim_target.normalized();
        
        let normal_cast = frame.actions_flags.intersects(ActionsFlags::NORMAL_CAST);

        let self_cast = frame.actions_flags.intersects(ActionsFlags::SELF_CAST);


        self.normal_cast = normal_cast;

        self.self_cast = self_cast

    }
    
    
    pub fn default_intent(&mut self){

        self.target = None;

        self.element_inputs = ElementsFlags::empty();

        self.normal_cast = false;

        self.self_cast = false

    }


    pub fn use_last_intent(&mut self) {
        
        self.target = None;

        self.element_inputs = ElementsFlags::empty();
    }


}
