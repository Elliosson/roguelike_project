extern crate specs;
use crate::{
    gamelog::{GameLog, WorldStatLog},
    Name, Renderable, Specie, TemperatureSensitive,
};
use rltk::RGB;
use specs::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

pub struct SpecieSystem {}

impl<'a> System<'a> for SpecieSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, WorldStatLog>,
        WriteStorage<'a, Specie>,
        WriteStorage<'a, TemperatureSensitive>,
        WriteStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, _names, _world_logs, mut species, temp_sensis, mut renderables) = data;

        let mut species_hash: HashMap<String, Vec<Entity>> = HashMap::new();

        let max_specie_member = 100;

        //create an hash map of all the member of all specie
        for (entity, specie, _temp_sens) in (&entities, &mut species, &temp_sensis).join() {
            if species_hash.contains_key(&specie.name) {
                let member_list = species_hash.get_mut(&specie.name).unwrap();
                member_list.push(entity);
            } else {
                species_hash.insert(specie.name.clone(), vec![entity]);
            }
        }

        let mut new_species: Vec<Vec<(Entity, f32)>> = Vec::new();

        //For now just divide the specie acording to the number of member
        for (_name, member_list) in &species_hash {
            if member_list.len() > max_specie_member {
                println!("Divide the specie");
                //divide the specie in two species, for knwo just acording to temperature optimum
                let mut opti_list: Vec<(Entity, f32)> = Vec::new();

                //for the memeber according to their optimum temp
                for ent in member_list.iter() {
                    opti_list.push((*ent, temp_sensis.get(*ent).unwrap().optimum));
                    opti_list.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(Ordering::Equal));

                    //slip
                }

                let half_list = opti_list.split_off(member_list.len() / 2);
                new_species.push(opti_list);
                new_species.push(half_list);
            }
        }

        //change the id of new_species, // also change their appearance
        //also change the name once the name is added to the mutation
        let mut count = 0;
        for new_specie in new_species {
            count += 1;
            for (entity, _opti) in new_specie {
                let mut specie = species.get_mut(entity).unwrap();

                //concatenate ne old name with a number
                specie.name = specie.name.clone() + &count.to_string();

                //change the color of the new specie todo get random color
                let mut renderable = renderables.get_mut(entity).unwrap();
                renderable.fg = RGB::named(rltk::BLUE);
            }
        }
    }
}
