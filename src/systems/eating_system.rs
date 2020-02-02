extern crate specs;
use crate::{gamelog::GameLog, Cow, EnergyReserve, Hunger, Leaf, ToDelete, WantToEat};
use specs::prelude::*;

pub struct EatingSystem {}

impl<'a> System<'a> for EatingSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantToEat>,
        WriteStorage<'a, Cow>,
        WriteStorage<'a, Leaf>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, ToDelete>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut log,
            mut want_to_eats,
            mut cows,
            mut leafs,
            mut energy_reserves,
            mut to_deletes,
        ) = data;

        let mut eated_leafs: Vec<Entity> = Vec::new();

        //resolve eating
        for (_entity, want_to_eat, mut en_res) in
            (&entities, &want_to_eats, &mut energy_reserves).join()
        {
            if let Some(leaf) = leafs.get_mut(want_to_eat.target) {
                if en_res.hunger == Hunger::Hungry {
                    en_res.reserve += leaf.nutriments as f32; //TODO no control of max res for know
                    leaf.nutriments = 0; //TODO maybe do something proper to imediatly suppress the leaf
                    eated_leafs.push(want_to_eat.target);
                }
            }
            if let Some(_cow) = cows.get_mut(want_to_eat.target) {
                if en_res.hunger == Hunger::Hungry {
                    en_res.reserve += 200.0; //TODO add nutriment from cow;

                    //TODO do something to prevent double eat
                    //kill entity
                    to_deletes
                        .insert(want_to_eat.target, ToDelete {})
                        .expect("Unable to insert");

                    log.entries.insert(0, format!("A entity have been eated"));
                }
            }
        }

        want_to_eats.clear();

        for done in eated_leafs {
            leafs.remove(done);
        }
    }
}
