use crate::model::world::World;

pub fn run(world: &mut World){

    for player in world.players.values_mut() {

        player.intent.use_last_intent();

    }


}