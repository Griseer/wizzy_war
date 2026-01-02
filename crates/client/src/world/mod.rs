use bevy::prelude::*;

mod interpolate;
mod remote_world;

pub use remote_world::RemoteWorld;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RemoteWorld::default());
    }
}
