extern crate specs;
use crate::{gamelog::GameLog, EnergyReserve, Hunger};
use specs::prelude::*;

pub struct EnergySystem {}

impl<'a> System<'a> for EnergySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, EnergyReserve>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut energy_reserves) = data;

        for (_entity, mut en_res) in (&entities, &mut energy_reserves).join() {
            //consumption of energy
            en_res.reserve -= en_res.base_consumption;

            if en_res.reserve < (en_res.max_reserve / 2) {
                en_res.hunger = Hunger::Hungry;
            } else {
                en_res.hunger = Hunger::Full;
            }
        }
    }
}
