extern crate specs;
use crate::{
    gamelog::{GameLog, SpeciesInstantLog, WorldStatLog},
    Carnivore, CombatStats, Cow, Date, EnergyReserve, HumiditySensitive, Name, Renderable,
    Reproduction, Specie, Speed, TemperatureSensitive,
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
        ReadStorage<'a, Reproduction>,
        ReadExpect<'a, Date>,
        WriteExpect<'a, WorldStatLog>,
        ReadStorage<'a, Specie>,
        ReadStorage<'a, TemperatureSensitive>,
        ReadStorage<'a, Renderable>,
        WriteExpect<'a, SpeciesInstantLog>,
        ReadStorage<'a, HumiditySensitive>,
        ReadStorage<'a, Speed>,
        ReadStorage<'a, Cow>,
        ReadStorage<'a, Carnivore>,
        ReadStorage<'a, CombatStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            _log,
            _names,
            energy_reserves,
            reproductions,
            _date,
            mut world_logs,
            species,
            temp_sensis,
            renderables,
            mut species_log,
            hum_sensis,
            speeds,
            cows,
            carnivores,
            combat_stats,
        ) = data;

        let mut thresholds = Vec::new();
        let mut max_res = Vec::new();
        let mut consumptions = Vec::new();
        let mut birth_energy = Vec::new();
        let mut move_points_turn = Vec::new();

        //General stat
        for (_entity, energy_res, reprod, speed) in
            (&entities, &energy_reserves, &reproductions, &speeds).join()
        {
            thresholds.push(reprod.offset_threshold);
            birth_energy.push(reprod.birth_energy);
            max_res.push(energy_res.max_reserve);
            consumptions.push(energy_res.base_consumption);
            move_points_turn.push(speed.point_per_turn);
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

            let sum: i32 = move_points_turn.iter().sum();
            let mean = sum / (len as i32);
            let buf = format!("Mean move point: {}", mean);
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
            let mut hum_optimum = 0.0;
            let mut max_reserve = 0.0;
            let mut base_consumption = 0.0;
            let mut birth_energy = 0;
            let mut offset_threshold = 0;
            let mut move_point = 0;
            let mut move_point_turn = 0;
            let mut cow_digestion = 0.0;
            let mut carnivore_digestion = 0.0;
            let mut power = 0.0;
            let number = member_list.len();

            //Do the mean of the vamue of each caracteristique for the specie
            for member in member_list.iter() {
                let temp_sensi = temp_sensis.get(*member).unwrap();
                let hum_sensi = hum_sensis.get(*member).unwrap();
                let energy_reserve = energy_reserves.get(*member).unwrap();
                let reproduction = reproductions.get(*member).unwrap();
                let speed = speeds.get(*member).unwrap();
                let cow = cows.get(*member).unwrap();
                let carnivore = carnivores.get(*member).unwrap();
                let combat_stat = combat_stats.get(*member).unwrap();

                optimum += temp_sensi.optimum;
                hum_optimum += hum_sensi.optimum;
                max_reserve += energy_reserve.max_reserve;
                base_consumption += energy_reserve.base_consumption;
                birth_energy += reproduction.birth_energy;
                offset_threshold += reproduction.offset_threshold;
                move_point += speed.move_point;
                move_point_turn += speed.point_per_turn;
                cow_digestion += cow.digestion;
                carnivore_digestion += carnivore.digestion;
                power += combat_stat.power as f32;
            }

            optimum = optimum / number as f32;
            hum_optimum = hum_optimum / number as f32;
            max_reserve = max_reserve / number as f32;
            base_consumption = base_consumption / number as f32;
            birth_energy = birth_energy / number as u32;
            offset_threshold = offset_threshold / number as u32;
            move_point = move_point / number as i32;
            move_point_turn = move_point_turn / number as i32;
            cow_digestion = cow_digestion / number as f32;
            carnivore_digestion = carnivore_digestion / number as f32;
            power = power / number as f32;

            let mut string_vec = Vec::new();

            let buf = format!("{} members of {} ", member_list.len(), name);
            string_vec.push(buf);
            let buf = format!(
                "tmp opti: {:.1}, max eng: {:.1}, eng cmsp:{:.1}",
                optimum, max_reserve, base_consumption,
            );
            string_vec.push(buf);
            let buf = format!(
                "birth eng: {}, b offset: {}, hum_opti: {:.1}",
                birth_energy, offset_threshold, hum_optimum
            ); //TODO add the temperature sensibilité an reprod threshold
            string_vec.push(buf);
            let buf = format!(
                " v_digest: {:.1}, ca_digest: {:.1}, pwr: {:.1}",
                cow_digestion, carnivore_digestion, power
            );
            string_vec.push(buf);
            let buf = format!("move_point: {}, point turn {}", move_point, move_point_turn);
            string_vec.push(buf);
            species_log
                .entries
                .push((string_vec, renderable.fg, renderable.glyph));
        }
    }
}
