extern crate specs;
use crate::{gamelog::GameLog, EnergyReserve, Name, SoloReproduction};
use specs::prelude::*;

pub struct SoloReproductionSystem {}

impl<'a> System<'a> for SoloReproductionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, SoloReproduction>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut log, mut energy_reserves, solo_reproductions, names) = data;

        for (_entity, solo_reprod, mut energy_reserve, _names) in
            (&entities, &solo_reproductions, &mut energy_reserves, &names).join()
        {
            if energy_reserve.reserve >= solo_reprod.threshold {
                energy_reserve.reserve -= solo_reprod.cost;
                //divide TODO
                log.entries
                    .insert(0, format!("A entity is want to divide."));
            }
        }
    }
}
