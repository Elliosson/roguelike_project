extern crate specs;
use crate::{gamelog::GameLog, Leaf, Renderable, Tree};
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
        WriteExpect<'a, rltk::RandomNumberGenerator>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (entities, _log, trees, mut leafs, mut renderables, mut rng) = data;

        for (entity, _tree, mut renderable) in (&entities, &trees, &mut renderables).join() {
            //change color of the tree acording to leaf
            //todo very heavy to change
            if let Some(_leaf) = leafs.get(entity) {
                renderable.fg = RGB::named(rltk::GREEN);
            } else {
                renderable.fg = RGB::named(rltk::YELLOW);
                //a chance to regrow the leaf
                let regrow_roll = rng.roll_dice(1, 100);
                if regrow_roll == 1 {
                    leafs
                        .insert(entity, Leaf { nutriments: 100 })
                        .expect("Unable to insert");
                }
            }
        }
    }
}
