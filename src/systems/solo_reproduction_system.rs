extern crate specs;
use crate::{
    gamelog::GameLog, BirthForm, BirthRequetList, Date, EnergyReserve, HumiditySensitive,
    Mutations, Name, Position, Renderable, Reproduction, Specie, TemperatureSensitive, UniqueId,
    WantsToDuplicate,
};
use specs::prelude::*;

pub struct SoloReproductionSystem {}

impl<'a> System<'a> for SoloReproductionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, Reproduction>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, WantsToDuplicate>,
        WriteExpect<'a, BirthRequetList>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Date>,
        ReadStorage<'a, UniqueId>,
        ReadStorage<'a, TemperatureSensitive>,
        ReadStorage<'a, Specie>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, HumiditySensitive>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut log,
            mut energy_reserves,
            reproductions,
            names,
            mut _want_to_duplicates,
            mut birth_request_list,
            positions,
            date,
            unique_ids,
            temp_sensis,
            species,
            renderables,
            hum_sensis,
        ) = data;

        //TODO don't check the mutable componant and them later if the entity have it
        for (
            entity,
            solo_reprod,
            mut energy_reserve,
            name,
            position,
            id,
            temp_sensi,
            specie,
            renderable,
            hum_sensi,
        ) in (
            &entities,
            &reproductions,
            &mut energy_reserves,
            &names,
            &positions,
            &unique_ids,
            &temp_sensis,
            &species,
            &renderables,
            &hum_sensis,
        )
            .join()
        {
            if energy_reserve.reserve >= solo_reprod.threshold() as f32 {
                energy_reserve.reserve -= solo_reprod.cost() as f32;
                log.entries
                    .insert(0, format!("A entity is want to divide."));

                let form = BirthForm {
                    name: name.clone(),
                    parent: entity,
                    parent_id: id.get(),
                    male_parent: None,
                    male_parent_id: None,
                    date: date.get_date(),
                    position: position.clone(),
                };

                let mutations = Mutations {
                    reproduction: Some(solo_reprod.clone()),
                    energy_reserve: Some(energy_reserve.clone()),
                    temp_sensi: Some(temp_sensi.clone()),
                    hum_sensi: Some(hum_sensi.clone()),
                    specie: Some(specie.clone()),
                    renderable: Some(renderable.clone()),
                    speed: None,       //TODO suport for speed in solo mutation
                    herbivore: None,   //TODO suport for herbivore in solo mutation
                    carnivore: None,   //TODO suport for carni in solo mutation
                    combat_stat: None, //TODO suport for combat_stat in solo mutation
                };

                /*println!(
                    "Birth request: id:{}, reserve{}, thrshoild{}, cost{} ,x{}, y{} ",
                    id.get(),
                    energy_reserve.reserve,
                    solo_reprod.threshold(),
                    solo_reprod.cost(),
                    position.x,
                    position.y
                );*/

                birth_request_list.request(form, mutations);
            }
        }
    }
}
