extern crate specs;
use crate::{algo::*, ApplyMove, Cow, Leaf, Map, Position, RunState, Viewshed, WantToEat};
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
        ) = data;

        let mut targets_leaf: HashMap<Entity, Entity> = HashMap::new();

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

            //chose a leaf, for now it always the first one
            if !found_leaf.is_empty() {
                targets_leaf.insert(cow_entity, found_leaf[0]);
                //TODO Prevent multiple cow on the same target
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
                100, //Max step for search, TODO thonk of a way to automatically find an acceptable number
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
