extern crate specs;
use crate::{
    gamelog::GameLog, BirthForm, BirthRequetList, Date, EnergyReserve, Mutations, Name, Position,
    SoloReproduction, WantsToDuplicate, UniqueId
};
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
        WriteStorage<'a, WantsToDuplicate>,
        WriteExpect<'a, BirthRequetList>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Date>,
        ReadStorage<'a, UniqueId>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut log,
            mut energy_reserves,
            solo_reproductions,
            names,
            mut _want_to_duplicates,
            mut birth_request_list,
            positions,
            date,
            unique_ids,
        ) = data;

        for (entity, solo_reprod, mut energy_reserve, name, position, id) in (
            &entities,
            &solo_reproductions,
            &mut energy_reserves,
            &names,
            &positions,
            &unique_ids,
        )
            .join()
        {
            if energy_reserve.reserve >= solo_reprod.threshold {
                energy_reserve.reserve -= solo_reprod.cost;
                log.entries
                    .insert(0, format!("A entity is want to divide."));

                let form = BirthForm {
                    name: name.clone(),
                    parents: entity,
                    parent_id: id.get(),
                    date: date.get_date(),
                    position: position.clone(),
                };

                birth_request_list.request(form, Mutations::new());
                /*    want_to_duplicates
                .insert(entity, WantsToDuplicate {})
                .expect("Unable to insert");*/
            }
        }
    }
}
