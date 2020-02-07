extern crate specs;
use crate::{algo, ApplyMove, Map, MyTurn, Position, WantsToFlee};
use specs::prelude::*;
use std::time::Instant;

pub struct FleeAI {}

impl<'a> System<'a> for FleeAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, WantsToFlee>,
        WriteStorage<'a, Position>,
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, ApplyMove>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut turns, mut want_flee, positions, mut map, entities, mut apply_move) = data;

        let now = Instant::now();

        let mut turn_done: Vec<Entity> = Vec::new();
        for (entity, pos, flee) in (&entities, &positions, &want_flee).join() {
            turn_done.push(entity);
            let my_idx = map.xy_idx(pos.x, pos.y);
            map.populate_blocked();
            let flee_map =
                algo::DijkstraMap::new(map.width, map.height, &flee.indices, &*map, 100.0);
            let flee_target = algo::DijkstraMap::find_highest_exit(&flee_map, my_idx as i32, &*map);
            if let Some(flee_target) = flee_target {
                if !map.blocked[flee_target as usize] {
                    apply_move
                        .insert(
                            entity,
                            ApplyMove {
                                dest_idx: flee_target,
                            },
                        )
                        .expect("Unable to insert");
                    turn_done.push(entity);
                }
            }
        }

        println!("djiskra time = {}", now.elapsed().as_micros());

        want_flee.clear();

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}