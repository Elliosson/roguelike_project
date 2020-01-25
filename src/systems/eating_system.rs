extern crate specs;
use crate::{gamelog::GameLog, Cow, EnergyReserve, Hunger, Leaf, WantToEat};
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
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut want_to_eats, mut cows, mut leafs, mut energy_reserves) = data;

        let mut eated_leafs: Vec<Entity> = Vec::new();

        //resolve eating
        for (_entity, want_to_eat, mut _cow, mut en_res) in
            (&entities, &want_to_eats, &mut cows, &mut energy_reserves).join()
        {
            if let Some(leaf) = leafs.get(want_to_eat.target) {
                if en_res.hunger == Hunger::Hungry {
                    en_res.reserve += leaf.nutriments as f32; //TODO no control of max res for know
                    eated_leafs.push(want_to_eat.target);
                }
            }
        }

        want_to_eats.clear();

        for done in eated_leafs {
            leafs.remove(done);
        }
    }
}
