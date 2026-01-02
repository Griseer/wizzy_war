use bevy::prelude::*;
use game_core::PlayerId;

use crate::world::RemoteWorld;

#[derive(Component)]
pub struct PlayerVisual {
    pub id: PlayerId,
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn render_players(
    mut commands: Commands,
    world: Res<RemoteWorld>,
    mut query: Query<(Entity, &PlayerVisual, &mut Transform)>,
) {
    // actualizar existentes
    for (entity, visual, mut transform) in query.iter_mut() {
        if let Some(pos) = world.players.get(&visual.id) {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        } else {
            commands.entity(entity).despawn();
        }
    }

    // spawn faltantes
    for (id, pos) in world.players.iter() {
        if !query.iter().any(|(_, v, _)| v.id == *id) {
            commands.spawn((
                Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::splat(20.0)),
                    ..default()
                },
                Transform::from_xyz(pos.x, pos.y, 0.0),
                GlobalTransform::default(),
                PlayerVisual { id: *id },
            ));
        }
    }
}
