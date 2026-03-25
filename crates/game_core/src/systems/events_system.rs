use crate::model::{events::GameEvent, world::World};

fn event_system(world: &mut World) {

    for event in world.events.drain_current() {

        match event {

            _ => {}
        }
    }
}

