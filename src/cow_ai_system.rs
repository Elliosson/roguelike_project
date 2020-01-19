extern crate specs;
use super::{ApplyMove, Cow, Leaf, Map, Position, RunState, Viewshed, WantToEat, WantsToMelee};
use specs::prelude::*;
extern crate rltk;
use rltk::Point;
use std::collections::HashMap;

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
            mut viewsheds,
            mut cows,
            mut positions,
            leafs,
            mut _want_to_eats,
            mut apply_move,
        ) = data;

        let mut targets_leaf: HashMap<Entity, Entity> = HashMap::new();

        //check if there is a leaf on position of a cow
        for (cow_entity, mut viewshed, mut cow, mut pos) in
            (&entities, &mut viewsheds, &mut cows, &mut positions).join()
        {
            let idx = map.xy_idx(pos.x, pos.y);
            for thing in map.tile_content[idx].iter() {
                if let Some(leaf) = leafs.get(*thing) {
                    println!("The Cow is on the leaf");
                    cow.food += 100;
                }
            }
        }

        //Chose the leaf to go
        for (cow_entity, mut viewshed, cow, mut pos) in
            (&entities, &mut viewsheds, &cows, &mut positions).join()
        {
            //search for every leaf in the viewshed
            let mut found_leaf: Vec<Entity> = Vec::new();
            for visible_tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                for maybe_leaf in map.tile_content[idx].iter() {
                    if let Some(leaf) = leafs.get(*maybe_leaf) {
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

        //Creat path to the chosen leaf
        for (cow_ent, leaf_ent) in &targets_leaf {
            let pos = positions.get(*cow_ent).expect("No postion");
            let target_pos = positions.get(*leaf_ent).expect("No postion");

            let path = rltk::a_star_search(
                map.xy_idx(pos.x, pos.y) as i32,
                map.xy_idx(target_pos.x, target_pos.y) as i32,
                &mut *map,
            );

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

        targets_leaf.clear();

        /*
        for (entity, mut viewshed, cow, mut want_to_eat) in (&entities, &mut viewsheds, &cows, &mut want_to_eats).join() {
            if let Some(target_pos) = positions.get(want_to_eat.target)
            {
                    //todo
            }




        }


        //we run cow in the monster turn for now
        if *runstate != RunState::MonsterTurn { return; }
        for (entity, mut viewshed, cow, mut pos) in (&entities, &mut viewsheds, &cows, &mut positions).join() {
            // One cow AI
            //for now all in the same function
            let done = false;

            //go eat food
            if let Some(want_to_eat) = want_to_eats.get(entity){


                if let Some(target_pos) = positions.get(want_to_eat.target)
                {
                    //eat food
                    let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y)
                        , Point::new(target_pos.x, target_pos.y));
                    if distance < 0.5 {
                        //eat
                        if let Some(leaf) = leafs.get(want_to_eat.target){
                            //todo just brut number for now
                            cow.food += 100;
                        }
                        else{
                            end_eat.push(entity);
                        }



                    }
                    //go to food
                    else{
                        let path = rltk::a_star_search(
                            map.xy_idx(pos.x, pos.y) as i32,
                            map.xy_idx(target_pos.x, target_pos.y) as i32,
                            &mut *map
                        );
                        //move
                        if path.success && path.steps.len()>0 {
                            let mut idx = map.xy_idx(pos.x, pos.y);
                            map.blocked[idx] = false;
                            pos.x = path.steps[1] % map.width;
                            pos.y = path.steps[1] / map.width;
                            idx = map.xy_idx(pos.x, pos.y);
                            map.blocked[idx] = true;
                            viewshed.dirty = true;
                        }
                    }



                }
                else{
                    //todo end go to eat
                    end_eat.push(entity);
                }



            }
            else{
                //If no target will search for every leaf in the viewshed
                for visible_tile in viewshed.visible_tiles.iter(){
                    let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                    for enti in map.tile_content[idx]{
                        if let Some(leaf) = leafs.get(enti){
                            found_leaf.push(enti);
                        }
                    }
                }

                //chose a leaf, for now it always the first one
                if !found_leaf.is_empty(){
                    want_to_eats.insert(entity, WantToEat{target: found_leaf[0]}).expect("Unable to insert");
                }

            }



        }

        for done in end_eat{
            want_to_eats.remove(done);
        }

        for done in end_leaf{
            leafs.remove(done);
        }

        */
    }
}
