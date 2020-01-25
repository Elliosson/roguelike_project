extern crate specs;
use crate::{gamelog::{GameLog, WorldStatLog}, Date, EnergyReserve, Name, SoloReproduction, };
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
        ReadExpect<'a, Date>,
        WriteExpect<'a, WorldStatLog>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, _names, energy_reserves, solo_reproductions, _date, mut world_logs) = data;

        let mut thresholds = Vec::new();
        let mut max_res = Vec::new();
        let mut consumptions = Vec::new();
        let mut birth_energy = Vec::new();

        for (_entity, energy_res, solo_reprod) in
            (&entities, &energy_reserves, &solo_reproductions).join()
        {
            thresholds.push(solo_reprod.offset_threshold);
            birth_energy.push(solo_reprod.birth_energy);
            max_res.push(energy_res.max_reserve);
            consumptions.push(energy_res.base_consumption);
        }

        let len = thresholds.iter().len();
        if len > 0 {
            let sum: u32 = thresholds.iter().sum();
            let mean = sum / (len as u32);
            let buf = format!("Mean bith offset threshold: {}", mean);
            world_logs.entries.push(buf);

            let sum: f32 = max_res.iter().sum();
            let mean = sum / (len as f32);
            let buf = format!("Mean energy reserve: {}", mean);
            world_logs.entries.push(buf);

            let sum: f32 = consumptions.iter().sum();
            let mean = sum / (len as f32);
            let buf = format!("Mean energy consuption: {}", mean);
            world_logs.entries.push(buf);

            let sum: u32 = birth_energy.iter().sum();
            let mean = sum / (len as u32);
            let buf = format!("Mean birth energy: {}", mean);
            world_logs.entries.push(buf);
        }
    }
}
