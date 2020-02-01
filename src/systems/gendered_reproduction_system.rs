extern crate specs;
use crate::{
    gamelog::GameLog, BirthForm, BirthRequetList, Date, EnergyReserve, HumiditySensitive, Hunger,
    Map, Mutations, Name, Position, Renderable, SoloReproduction, Specie, TemperatureSensitive,
    UniqueId, Viewshed, WantsToDuplicate,
};
use specs::prelude::*;

pub struct GenderedReproductionSystem {}

impl<'a> System<'a> for GenderedReproductionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, SoloReproduction>, //TODO remplace by genderedReproduction
        ReadStorage<'a, Name>,
        WriteStorage<'a, WantsToDuplicate>,
        WriteExpect<'a, BirthRequetList>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Date>,
        ReadStorage<'a, UniqueId>,
        ReadStorage<'a, TemperatureSensitive>,
        ReadStorage<'a, Specie>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Viewshed>,
        ReadExpect<'a, Map>,
        ReadStorage<'a, HumiditySensitive>,
    );

    //TODO add male and femal
    //For now there is no real sex, the entity will just a search an entity of the same specy in the vicinity
    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut _log,
            mut energy_reserves,
            solo_reproductions,
            names,
            mut _want_to_duplicates,
            mut birth_request_list,
            positions,
            date,
            unique_ids,
            temp_sensis,
            species,
            renderables,
            viewsheds,
            map,
            hum_sensis,
        ) = data;

        //store all "female" entity that have sucessfully reproduce
        let mut have_reproduce: Vec<(Entity, f32)> = Vec::new();

        //search a male in the viewshed that can reproduce and are of the same specie
        //TODO probably supress solo reproduction
        for (
            entity,
            viewshed,
            specie,
            eng_res,
            temp_sensi,
            renderable,
            solo_reprod,
            name,
            position,
            id,
            hum_sensi,
        ) in (
            &entities,
            &viewsheds,
            &species,
            &energy_reserves,
            &temp_sensis,
            &renderables,
            &solo_reproductions,
            &names,
            &positions,
            &unique_ids,
            &hum_sensis,
        )
            .join()
        {
            //if the energy reserve are sufisiant to reproduce
            if eng_res.reserve >= solo_reprod.threshold() as f32 {
                //search a male in the viewshed that can reproduce and are of the same specie
                let mut possible_mates: Vec<Entity> = Vec::new();

                for visible_tile in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                    for maybe_mate in map.tile_content[idx].iter() {
                        //Check that the mate is not himself
                        if maybe_mate.id() != entity.id() {
                            if let Some(mate_specie) = species.get(*maybe_mate) {
                                if mate_specie.name == specie.name {
                                    possible_mates.push(*maybe_mate);
                                    //add in possible mate
                                }
                            }
                        }
                    }
                }
                //In all the male chose the one with the most energy
                //TODO add a minimum of energy
                let mut choosen_mate: Option<Entity> = None;
                let mut max_energy = 0.0;

                for mate in possible_mates.iter() {
                    if let Some(mate_energy) = energy_reserves.get(*mate) {
                        if mate_energy.reserve > max_energy {
                            choosen_mate = Some(*mate);
                            max_energy = mate_energy.reserve;
                        }
                    }
                }

                //If we have a choosen mate
                if let Some(my_mate) = choosen_mate {
                    //copy the components that can mutate and are not specific to the specie
                    let mate_energy = energy_reserves.get(my_mate).unwrap();
                    let mate_temp_sensi = temp_sensis.get(my_mate).unwrap();
                    let mate_hum_sensi = hum_sensis.get(my_mate).unwrap();
                    let mate_id = unique_ids.get(my_mate).unwrap();

                    //Construct the Mutations struct with the median of all the component of both parent
                    //TODO create comparaison operator for the components
                    //For now we handle this the ugly way by doing a median on the componant that interest us

                    let new_temp_sensi = TemperatureSensitive {
                        optimum: (temp_sensi.optimum + mate_temp_sensi.optimum) / 2.0,
                        k: (temp_sensi.k + mate_temp_sensi.k) / 2.0,
                    };
                    let new_hum_sensi = HumiditySensitive {
                        optimum: (hum_sensi.optimum + mate_hum_sensi.optimum) / 2.0,
                        k: (hum_sensi.k + mate_hum_sensi.k) / 2.0,
                    };
                    let new_energy_res = EnergyReserve {
                        reserve: 0.0, //No heritance
                        max_reserve: (eng_res.max_reserve + mate_energy.max_reserve) / 2.0,
                        base_consumption: 0.0, //No heritance
                        hunger: Hunger::Full,  //No heritance
                    };
                    let mutations = Mutations {
                        solo_reproduction: Some(solo_reprod.clone()), //TODO Supress ? For now just inhereite from mother
                        energy_reserve: Some(new_energy_res),
                        temp_sensi: Some(new_temp_sensi),
                        hum_sensi: Some(new_hum_sensi),
                        specie: Some(specie.clone()),
                        renderable: Some(renderable.clone()),
                    };

                    //create birth
                    let form = BirthForm {
                        name: name.clone(),
                        parent: entity,
                        parent_id: id.get(),
                        male_parent: Some(my_mate),
                        male_parent_id: Some(mate_id.get()),
                        date: date.get_date(),
                        position: position.clone(),
                    };
                    //Send the birth request the classical way
                    birth_request_list.request(form, mutations);

                    //vec to consume the energy of the reproduction later
                    have_reproduce.push((entity, solo_reprod.cost() as f32));
                }
            }
        }

        //Consume the energy of the reproduction
        //TODO it's no perfect because the entiy can be brought in an interaction before the energy is effectly removed
        //It will be resolved when we will only have male but it's still dangerous
        //TODO check if get_mut permet bien de mut le component
        for (entity, cost) in have_reproduce {
            let energy_reserve = energy_reserves.get_mut(entity).unwrap();
            energy_reserve.reserve -= cost;
        }
    }
}
