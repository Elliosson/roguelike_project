extern crate specs;
use crate::{gamelog::GameLog, Aging, ToDelete};
use specs::prelude::*;

pub struct AgingSystem {}

impl<'a> System<'a> for AgingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Aging>,
        WriteStorage<'a, ToDelete>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut agings, mut to_deletes) = data;

        //For now just kill when the creature reach the life expetancy
        for (entity, aging) in (&entities, &mut agings).join() {
            aging.age += 1;
            if aging.age > aging.life_expectancy {
                //kill him
                to_deletes
                    .insert(entity, ToDelete {})
                    .expect("Unable to insert");

                log.entries
                    .insert(0, format!("A entity is dead of  old age."));
            }
        }
    }
}
