extern crate specs;
use crate::{gamelog::GameLog, EnergyReserve, Hunger, ToDelete};
use specs::prelude::*;

pub struct EnergySystem {}

impl<'a> System<'a> for EnergySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, ToDelete>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut energy_reserves, mut to_deletes) = data;

        for (entity, mut en_res) in (&entities, &mut energy_reserves).join() {
            //consumption of energy
            en_res.reserve -= en_res.base_consumption;

            if en_res.reserve <= 0.0 {
                //kill entity
                to_deletes
                    .insert(entity, ToDelete {})
                    .expect("Unable to insert");

                log.entries
                    .insert(0, format!("A entity is dead of starvation."));
            } else if en_res.reserve < (en_res.max_reserve / 2.0) {
                en_res.hunger = Hunger::Hungry;
            } else {
                en_res.hunger = Hunger::Full;

                //reproduce
            }
        }
    }
}
