extern crate specs;
use crate::{gamelog::GameLog, Cow, Leaf, Renderable, WantToEat};
use rltk::RGB;
use specs::prelude::*;

pub struct EnergySystem {}

impl<'a> System<'a> for EnergySystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantToEat>,
        WriteStorage<'a, Cow>,
        WriteStorage<'a, Leaf>,
        WriteStorage<'a, Renderable>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, mut want_to_eats, mut cows, mut leafs, mut renderables) = data;
    }
}