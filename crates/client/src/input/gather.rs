use bevy::prelude::*;

pub fn gather_input(keyboard: Res<ButtonInput<KeyCode>>, mut writer: MessageWriter<MoveInput>) {
    let mut dir = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        dir.y += 1.0;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        dir.y -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        dir.x -= 1.0;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        dir.x += 1.0;
    }

    if dir != Vec2::ZERO {
        writer.send(MoveInput {
            direction: dir.normalize(),
        });
    }
}
