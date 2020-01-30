extern crate specs;
use crate::{
    gamelog::{GameLog, SpeciesInstantLog, WorldStatLog},
    Date, EnergyReserve, Name, Renderable, SoloReproduction, Specie, TemperatureSensitive,
};
use specs::prelude::*;
use std::collections::BTreeMap;

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
        WriteExpect<'a, SpeciesInstantLog>,
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
            mut species_log,
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

        let mut species_hash: BTreeMap<String, Vec<Entity>> = BTreeMap::new();

        //create an hash map of all the member of all species
        for (entity, specie) in (&entities, &species).join() {
            if species_hash.contains_key(&specie.name) {
                let member_list = species_hash.get_mut(&specie.name).unwrap();
                member_list.push(entity);
            } else {
                species_hash.insert(specie.name.clone(), vec![entity]);
            }
        }

        species_log.entries.clear();
        //print all the species and the number of their members
        for (name, member_list) in &species_hash {
            let renderable = renderables.get(member_list[0]).unwrap(); //should not be possible to have 0 members, still ugly

            let mut optimum = 0.0;
            let mut max_reserve = 0.0;
            let mut base_consumption = 0.0;
            let mut birth_energy = 0;
            let mut offset_threshold = 0;
            let number = member_list.len();

            //Do the mean of the vamue of each caracteristique for the specie
            for member in member_list.iter() {
                let temp_sensi = temp_sensis.get(*member).unwrap();
                let energy_reserve = energy_reserves.get(*member).unwrap();
                let solo_reproduction = solo_reproductions.get(*member).unwrap();

                optimum += temp_sensi.optimum;
                max_reserve += energy_reserve.max_reserve;
                base_consumption += energy_reserve.base_consumption;
                birth_energy += solo_reproduction.birth_energy;
                offset_threshold += solo_reproduction.offset_threshold;
            }

            optimum = optimum / number as f32;
            max_reserve = max_reserve / number as f32;
            base_consumption = base_consumption / number as f32;
            birth_energy = birth_energy / number as u32;
            offset_threshold = offset_threshold / number as u32;

            let mut string_vec = Vec::new();

            let buf = format!("{} members of {} ", member_list.len(), name);
            string_vec.push(buf);
            let buf = format!(
                "tmp opti: {:.1}, max eng: {:.1}, eng cmsp:{:.1}",
                optimum, max_reserve, base_consumption,
            );
            string_vec.push(buf);
            let buf = format!(
                "birth eng: {}, b offset: {}",
                birth_energy, offset_threshold,
            ); //TODO add the temperature sensibilité an reprod threshold
            string_vec.push(buf);
            species_log
                .entries
                .push((string_vec, renderable.fg, renderable.glyph));
        }
    }
}
