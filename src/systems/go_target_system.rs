extern crate specs;
use crate::{
    algo, BlocksTile, EntityMoved, GoOnTarget, Map, Position, RunState, Speed, TargetReached,
    Viewshed,
};
use specs::prelude::*;

pub struct GoTargetSystem {}

impl<'a> System<'a> for GoTargetSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, BlocksTile>,
        Entities<'a>,
        WriteStorage<'a, EntityMoved>,
        WriteStorage<'a, Viewshed>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, RunState>,
        WriteStorage<'a, GoOnTarget>,
        WriteStorage<'a, Speed>,
        WriteStorage<'a, TargetReached>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            mut positions,
            blockers,
            entities,
            mut moved,
            mut viewsheds,
            _player_entity,
            mut _runstate,
            mut go_targets,
            mut speeds,
            mut target_reachs,
        ) = data;

        target_reachs.clear();

        //regain move point
        for (_entity, speed) in (&entities, &mut speeds).join() {
            speed.move_point += speed.point_per_turn;
        }

        for (entity, go_target, speed) in (&entities, &go_targets, &mut speeds).join() {
            println!("go target");
            let mut path;
            {
                let pos = positions.get(entity).expect("No postion");
                let target_pos = positions.get(go_target.target).expect("No postion");

                //creat a clone of the map with the destination ubvlocked, if not he will never find a way to go on the target
                let mut path_map = map.clone();
                let dest_idx = path_map.xy_idx(target_pos.x, target_pos.y);
                path_map.blocked[dest_idx] = false;

                path = algo::a_star_search(
                    path_map.xy_idx(pos.x, pos.y) as i32,
                    path_map.xy_idx(target_pos.x, target_pos.y) as i32, //TODO change that, the "-1" is a dirty fix for the imposibility to go on a blicked tile
                    &mut path_map,
                    200, //Max step for search, TODO thonk of a way to automatically find an acceptable number
                );
            }

            //Move for Real
            //TODO I need to resolve herbivore movement before carnivor movement, but even so it's not perfect
            if path.success {
                //I am not sure the pathfind can find a way if the target is blocking entity, aparently, no
                println!("path sucees");
                let pos = positions.get_mut(entity).expect("No postion");
                if path.steps.len() > 1 {
                    path.steps.remove(0); //we remove the initial position

                    for (vec_idx, dest_idx) in path.steps.iter().enumerate() {
                        //we are in the last iteration
                        if vec_idx >= path.steps.len() - 1 {
                            println!("on target1");
                            println!("send target reached");
                            //we are in contact //TODO find a way to clean target_reachs
                            target_reachs
                                .insert(
                                    entity,
                                    TargetReached {
                                        target: go_target.target,
                                    },
                                )
                                .expect("unable to insert");
                            break;
                        }
                        //100 move point per tilde for now
                        let move_cost = 100;
                        if speed.move_point >= move_cost {
                            //aply move
                            let start_idx = map.xy_idx(pos.x, pos.y);
                            let is_blocking = blockers.get(entity);
                            if is_blocking.is_some() {
                                map.blocked[start_idx] = false;
                                map.blocked[*dest_idx as usize] = true;
                            }
                            pos.x = dest_idx % map.width;
                            pos.y = dest_idx / map.width;
                            if let Some(vs) = viewsheds.get_mut(entity) {
                                vs.dirty = true;
                            }
                            moved
                                .insert(entity, EntityMoved {})
                                .expect("Unable to insert"); //this should not be usefull anymore
                            speed.move_point -= move_cost;
                        } else {
                            break; //not enought move point
                        }
                    }
                } else {
                    println!("on target2");
                    //we are in contact
                    target_reachs
                        .insert(
                            entity,
                            TargetReached {
                                target: go_target.target,
                            },
                        )
                        .expect("unable to insert");
                }
            } else {
                println!("path failed");
            }
        }

        go_targets.clear();
    }
}
