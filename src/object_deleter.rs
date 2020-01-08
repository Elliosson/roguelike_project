extern crate specs;
use specs::prelude::*;
use super::{ToDelete, gamelog::GameLog, RunState};


pub fn delete_entity_to_delete(ecs : &mut World) {
    
    let mut dead : Vec<Entity> = Vec::new();
    // Using a scope to make the borrow checker happy
    {
        let mut to_deletes = ecs.write_storage::<ToDelete>();
        let mut entities = ecs.entities();
        let mut log = ecs.write_resource::<GameLog>();

        for (entity, _to_delete) in (&entities, &to_deletes).join() {
            dead.push(entity);
        }
    }

    for victim in dead {
        ecs.delete_entity(victim).expect("Unable to delete");
    }
}
