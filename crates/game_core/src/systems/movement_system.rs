use shared::math::Vec2f;

use crate::model::{
    player::{Player},
    world::World,
};

pub fn run(world: &mut World, dt: f32) {
    for player in world.players.values_mut() {

        set_velocity(player, dt);
    }
}


fn set_velocity(player: &mut Player, dt: f32) {
    //let body = &mut player.physic_body;


    // if let Some(physic_effect) = &player.physic_effect {
    //     body.velocity = physic_effect.velocity;
    //     return;
    // }

    
    // let speed_mult = compute_speed_multiplier(player);

    // if speed_mult == 0.0 {
    //     body.velocity = Vec2f::ZERO;
    //     return;
    // }

    
    let new_velocity = compute_movement_velocity(player, dt);

    player.physic_body.velocity = new_velocity * 1.0;
}


fn compute_movement_velocity(player: &mut Player, dt: f32) -> Vec2f {
    const ARRIVAL_EPSILON: f32 = 0.05;
    const ACCEL: f32 = 20.0;
    const SLOW_RADIUS: f32 = 1.5;

    let body = &player.physic_body;

    let Some(target) = player.state.target else {
        return Vec2f::ZERO;
    };

    let pos = body.position;
    let to_target = target - pos;
    let distance = to_target.length();

    if distance <= ARRIVAL_EPSILON {
        return Vec2f::ZERO;
    }

    let desired_dir = to_target.normalized();

    let desired_speed = if distance < SLOW_RADIUS {
        player.build.speed * (distance / SLOW_RADIUS)
    } else {
        player.build.speed
    };

    let desired_vel = desired_dir * desired_speed;

    let vel_delta = desired_vel - body.velocity;
    let max_delta = ACCEL * dt;

    body.velocity + vel_delta.clamp_length(max_delta)
}


fn compute_speed_multiplier(player: &Player) -> f32 {


    1.0
}




fn set_target(player: &mut Player) {
    if let Some(target) = player.intent.target {
        player.state.target = Some(target);
    }
}