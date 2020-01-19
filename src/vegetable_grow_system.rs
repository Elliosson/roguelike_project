extern crate specs;
use super::{gamelog::GameLog, Leaf, Renderable, Tree};
use rltk::RGB;
use specs::prelude::*;

pub struct VegetableGrowSystem {}

impl<'a> System<'a> for VegetableGrowSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, Tree>,
        WriteStorage<'a, Leaf>,
        WriteStorage<'a, Renderable>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, trees, leafs, mut renderables) = data;

        for (entity, _tree, mut renderable) in (&entities, &trees, &mut renderables).join() {
            //change color of the tree acording to leaf
            //todo very heavy to change
            if let Some(_leaf) = leafs.get(entity) {
                renderable.fg = RGB::named(rltk::GREEN);
            } else {
                renderable.fg = RGB::named(rltk::YELLOW);
            }
        }
    }
}
