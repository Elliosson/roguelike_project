extern crate specs;
use crate::{
    algo::*, ApplyMove, Cow, Leaf, Map, Point, Position, RunState, TargetedForEat, Viewshed,
    WantToEat,
};
use specs::prelude::*;
extern crate rltk;
use std::collections::HashMap;
//use std::time::{Duration, Instant};

pub struct CowAI {}

impl<'a> System<'a> for CowAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Cow>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Leaf>,
        WriteStorage<'a, WantToEat>,
        WriteStorage<'a, ApplyMove>,
        WriteStorage<'a, TargetedForEat>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            _runstate,
            entities,
            viewsheds,
            cows,
            mut positions,
            leafs,
            mut want_to_eats,
            mut apply_move,
            mut targeted_eats,
        ) = data;

        let mut targets_leaf: HashMap<Entity, Entity> = HashMap::new();

        targeted_eats.clear(); //TODO dirty, create a system specificaly to clear this.

        //check if there is a leaf on position of a cow
        for (cow_entity, _cow, pos) in (&entities, &cows, &mut positions).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            for thing in map.tile_content[idx].iter() {
                if let Some(_leaf) = leafs.get(*thing) {
                    want_to_eats
                        .insert(cow_entity, WantToEat { target: *thing })
                        .expect("Unable to insert");
                }
            }
        }

        //Chose the leaf to go
        for (cow_entity, viewshed, _cow) in (&entities, &viewsheds, &cows).join() {
            //search for every leaf in the viewshed
            let mut found_leaf: Vec<Entity> = Vec::new();
            for visible_tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                for maybe_leaf in map.tile_content[idx].iter() {
                    if let Some(_leaf) = leafs.get(*maybe_leaf) {
                        found_leaf.push(*maybe_leaf);
                    }
                }
            }

            let mut choosen_leaf: Option<Entity> = None;
            let mut min: f32 = std::f32::MAX;
            for leaf in found_leaf {
                let leaf_pos = positions.get(leaf).unwrap();
                let pos = positions.get(cow_entity).unwrap();
                let maybe_targeted_eat = targeted_eats.get(leaf);

                //if their is a other creature that want the target, then I only go if I am closer
                let mut competitor_distance = std::f32::MAX;
                if let Some(targeted) = maybe_targeted_eat {
                    competitor_distance = targeted.distance;
                }
                let distance = rltk::DistanceAlg::Pythagoras
                    .distance2d(Point::new(pos.x, pos.y), Point::new(leaf_pos.x, leaf_pos.y));
                if (distance < min) && (distance < competitor_distance) {
                    choosen_leaf = Some(leaf);
                    min = distance;
                }
            }
            if let Some(leaf) = choosen_leaf {
                targets_leaf.insert(cow_entity, leaf);
                targeted_eats
                    .insert(
                        leaf,
                        TargetedForEat {
                            predator: cow_entity,
                            distance: min,
                        },
                    )
                    .expect("Unable ot insert");
            }
        }
        //println!("Start A*");
        //let now2 = Instant::now();

        //Creat path to the chosen leaf
        for (cow_ent, leaf_ent) in &targets_leaf {
            let pos = positions.get(*cow_ent).expect("No postion");
            let target_pos = positions.get(*leaf_ent).expect("No postion");

            //let now = Instant::now();

            let path = a_star_search(
                map.xy_idx(pos.x, pos.y) as i32,
                map.xy_idx(target_pos.x, target_pos.y) as i32,
                &mut *map,
                100, //Max step for search, TODO think of a way to automatically find an acceptable number
            );

            //println!("a* time = {}", now.elapsed().as_micros());

            //move
            if path.success && path.steps.len() > 1 {
                apply_move
                    .insert(
                        *cow_ent,
                        ApplyMove {
                            dest_idx: path.steps[1],
                        },
                    )
                    .expect("Unable to insert");
            }
        }

        //println!("Total a* time = {}", now2.elapsed().as_micros());

        targets_leaf.clear();
    }
}
