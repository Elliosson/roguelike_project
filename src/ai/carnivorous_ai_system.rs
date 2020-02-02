extern crate specs;
use crate::{
    Carnivore, Cow, GoOnTarget, Map, Point, Position, RunState, TargetReached, Viewshed, WantToEat,
    WantsToFlee,
};
use specs::prelude::*;
extern crate rltk;
use std::collections::HashMap;
//use std::time::{Duration, Instant};

pub struct CarnivorousAI {}

impl<'a> System<'a> for CarnivorousAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, RunState>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Cow>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantToEat>,
        WriteStorage<'a, Carnivore>,
        WriteStorage<'a, TargetReached>,
        WriteStorage<'a, GoOnTarget>,
        WriteStorage<'a, WantsToFlee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            _runstate,
            entities,
            viewsheds,
            cows,
            mut positions,
            mut want_to_eats,
            carnivores,
            mut target_reacheds,
            mut go_targets,
            mut flees,
        ) = data;

        //TODO add hunger condition to hunt

        let mut targets: HashMap<Entity, Entity> = HashMap::new();

        //check if we managed to get a target
        for (entity, _carnivore, _pos) in (&entities, &carnivores, &mut positions).join() {
            println!("in");
            if let Some(reached) = target_reacheds.get(entity) {
                println!("send want to eat");
                //TODO for now it eat directly I must add a fight
                want_to_eats
                    .insert(
                        entity,
                        WantToEat {
                            target: reached.target,
                        },
                    )
                    .expect("Unable to insert");
            //TODO do not search a new target if the entity is already eating
            } else {
                println!("no target reached");
            }
        }
        target_reacheds.clear();

        //Chose target to go, for now it's just Cow
        //TODO supress cow to have all sort of target
        for (entity, viewshed, _carnivore) in (&entities, &viewsheds, &carnivores).join() {
            println!("in2");
            //search for every cow in the viewshed
            let mut found_cows: Vec<Entity> = Vec::new();
            for visible_tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                for maybe_cow in map.tile_content[idx].iter() {
                    if let Some(_cow) = cows.get(*maybe_cow) {
                        found_cows.push(*maybe_cow);
                    }
                }
            }

            //Choose the closer target
            let mut choosen_cow: Option<Entity> = None;
            let mut min: f32 = std::f32::MAX;
            for cow in found_cows {
                let cow_pos = positions.get(cow).unwrap();
                let pos = positions.get(entity).unwrap();
                let distance = rltk::DistanceAlg::Pythagoras
                    .distance2d(Point::new(pos.x, pos.y), Point::new(cow_pos.x, cow_pos.y));
                if distance < min {
                    choosen_cow = Some(cow);
                    min = distance;
                }
            }
            if let Some(choosen_target) = choosen_cow {
                targets.insert(entity, choosen_target);
                println!("go target");
                go_targets
                    .insert(
                        entity,
                        GoOnTarget {
                            target: choosen_target,
                        },
                    )
                    .expect("Unable to insert");

                //tell the target to flee //TODO do a real system with done insert and all
                let pos = positions.get(entity).unwrap();
                let idx = map.xy_idx(pos.x, pos.y) as i32;
                let mut flee_list = Vec::new();
                flee_list.push(idx);

                flees
                    .insert(choosen_target, WantsToFlee { indices: flee_list })
                    .expect("Unable to insert");
            }
        }
    }
}
