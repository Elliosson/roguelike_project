extern crate specs;
use crate::{gamelog::GameLog, EnergyReserve, Name, SoloReproduction};
use specs::prelude::*;

pub struct StatSystem {}

impl<'a> System<'a> for StatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        ReadStorage<'a, EnergyReserve>,
        ReadStorage<'a, SoloReproduction>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, _names, energy_reserves, solo_reproductions) = data;

        let mut thresholds = Vec::new();

        for (_entity, _energy_res, solo_reprod) in
            (&entities, &energy_reserves, &solo_reproductions).join()
        {
            thresholds.push(solo_reprod.threshold)
        }

        let len = thresholds.iter().len();
        if len > 0 {
            let sum: i32 = thresholds.iter().sum();
            let mean = sum / (thresholds.iter().len() as i32);
            println!("Mean bith threshold: {}", mean);
        }
    }
}
