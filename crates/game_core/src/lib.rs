// game_core


// Client  -> InputFrame -> PlayerCommand
// Server  -> apply_command
// Core    -> simulate_world
// Server  -> snapshot
// Client  -> render

//
pub mod model;
pub mod systems;
pub mod snapshot;

use model::world::*;

use systems::*;


pub fn create_world() -> World {
    World::new()
}

pub fn simulate_world( world: &mut World, dt:f32, current_tick:u32 ) {
    
    
    
    //event_system
    intent_system::run(world);
    casting_system::run(world);
    //spawn_system::run(world, dt);
    //effect_system::run(world);
    //steering_system::run(world, dt);
    //projectile_system::run(world, dt);
    physics_system::run(world, dt);
    //collision_system::run(world);
    //damage_system::run(world);
    //death_system::run(world);
    cleanup_system::run(world);
    
    
    //  ├─ event_system
    //  ├─ input_system
    //  ├─ intent_system
    //  ├─ cast_system
    //  ├─ spawn_system
    //  ├─ status_system
    //  ├─ steering_system
    //  ├─ physics_system
    //  ├─ projectile_system
    //  ├─ beam_system
    //  ├─ collision_system
    //  ├─ element_interaction_system
    //  ├─ damage_system
    //  ├─ death_system
    //  ├─ cleanup_system
    //  └─ events.swap()

}