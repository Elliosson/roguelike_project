extern crate specs;
use crate::{
    gamelog::{GameLog, WorldStatLog},
    Date, EnergyReserve, Name, Renderable, SoloReproduction, Specie, TemperatureSensitive,
};
use specs::prelude::*;
use std::collections::HashMap;

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
        ReadStorage<'a, Specie>,
        ReadStorage<'a, TemperatureSensitive>,
        ReadStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            _names,
            energy_reserves,
            solo_reproductions,
            _date,
            mut world_logs,
            species,
            temp_sensis,
            renderables,
        ) = data;

        let mut thresholds = Vec::new();
        let mut max_res = Vec::new();
        let mut consumptions = Vec::new();
        let mut birth_energy = Vec::new();

        //General stat
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

        let mut species_hash: HashMap<String, Vec<Entity>> = HashMap::new();

        //create an hash map of all the member of all species
        for (entity, specie) in (&entities, &species).join() {
            if species_hash.contains_key(&specie.name) {
                let member_list = species_hash.get_mut(&specie.name).unwrap();
                member_list.push(entity);
            } else {
                species_hash.insert(specie.name.clone(), vec![entity]);
            }
        }

        //print all the species and the number of their members
        for (name, member_list) in &species_hash {
            let renderable = renderables.get(member_list[0]).unwrap(); //should not be possible to have 0 members, still ugly
            let temp_sensi = temp_sensis.get(member_list[0]).unwrap(); //should not be possible to have 0 members, still ugly
            let energy_reserve = energy_reserves.get(member_list[0]).unwrap(); //should not be possible to have 0 members, still ugly
            let solo_reproduction = solo_reproductions.get(member_list[0]).unwrap(); //should not be possible to have 0 members, still ugly
            println!(
                "Their is {} members of the specie {} {}\n tmp opti: {}, max eng: {}, eng cmsp:{}\n birth eng: {}, b offset: {}",
                member_list.len(),
                name,
                renderable.glyph as char,
                temp_sensi.optimum,
                energy_reserve.max_reserve,
                energy_reserve.base_consumption,
                solo_reproduction.birth_energy,
                solo_reproduction.offset_threshold,
            ); //TODO add the temperature sensibilité an reprod threshold
        }
    }
}
