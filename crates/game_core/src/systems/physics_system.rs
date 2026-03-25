use crate::model::world::World;

pub fn run(world: &mut World,dt:f32) {
    
    
    for player in world.players.values_mut() {
        player.physic_body.position += player.physic_body.velocity * dt;
    }

    // for projectile in world.projectiles.values_mut() {
    //     projectile.physic_body.position += projectile.physic_body.velocity * dt;
    // }
    
}