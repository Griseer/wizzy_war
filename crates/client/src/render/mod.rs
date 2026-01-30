use bevy::prelude::*;
use crate::world::components::PlayerEntity;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
        app.add_systems(Update, attach_sprite_to_players);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default(),
        GlobalTransform::default(),
    ));
}

fn attach_sprite_to_players(
    mut commands: Commands,
    query: Query<Entity, (With<PlayerEntity>, Without<Sprite>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            Sprite {
                color: Color::srgb(0.2, 0.9, 1.0),
                custom_size: Some(Vec2::splat(20.0)),
                ..default()
            },
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Visible,
            InheritedVisibility::default(),
        ));
    }
}