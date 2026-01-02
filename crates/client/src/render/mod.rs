use bevy::prelude::*;

mod players;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, players::setup_camera)
            .add_systems(Update, players::render_players);
    }
}
