
pub enum ElementalEffectType{
    Wet,
    Cold,
    Burn,
    Frozen,
}

pub struct ElementalEffect{

    pub effect: ElementalEffectType,
    pub start_tick: u32,
    pub end_tick:u32,


}



impl ElementalEffect {
    
    fn new(effect: ElementalEffectType, current_tick:u32) -> Self{

        // 60 tick = 1 sec
        match effect {
            ElementalEffectType::Wet => {
                Self{
                    effect: ElementalEffectType::Wet,
                    start_tick: current_tick,
                    end_tick: current_tick + 300,
                }
            }

            ElementalEffectType::Burn =>{
                Self{
                    effect: ElementalEffectType::Burn,
                    start_tick: current_tick + 60, // + 1 sec
                    end_tick: current_tick + 300, // + 5 sec
                }
            }

            ElementalEffectType::Cold =>{
                Self{
                    effect: ElementalEffectType::Cold,
                    start_tick: current_tick,
                    end_tick: current_tick + 1,
                }
            }
            
            ElementalEffectType::Frozen =>{
                Self{
                    effect: ElementalEffectType::Frozen,
                    start_tick: current_tick,
                    end_tick: current_tick + 300,
                }
            }

        }

    }
}