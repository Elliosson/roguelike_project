extern crate specs;
use crate::{
    Animal, Carnivore, CombatStats, Cow, EnergyReserve, GoOnTarget, Hunger, Leaf, Map,
    MyChoosenFood, MyTurn, Point, Position, RunState, SearchScope, Specie, TargetReached,
    TargetedForEat, Viewshed, WantToEat, WantsToFlee,
};
use specs::prelude::*;
extern crate rltk;

//use std::time::{Duration, Instant};

pub struct OmnivoreAI {}

impl<'a> System<'a> for OmnivoreAI {
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
        WriteStorage<'a, TargetedForEat>,
        WriteStorage<'a, GoOnTarget>,
        WriteStorage<'a, TargetReached>,
        WriteStorage<'a, Specie>,
        WriteStorage<'a, Animal>,
        WriteStorage<'a, Carnivore>,
        WriteStorage<'a, WantsToFlee>,
        WriteStorage<'a, EnergyReserve>,
        WriteStorage<'a, MyTurn>,
        WriteStorage<'a, MyChoosenFood>,
        WriteStorage<'a, CombatStats>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            map,
            _runstate,
            entities,
            viewsheds,
            cows,
            mut positions,
            leafs,
            mut want_to_eats,
            mut targeted_eats,
            mut go_targets,
            target_reacheds,
            species,
            animals,
            carnivores,
            mut flees,
            energy_reserves,
            mut turns,
            mut my_choosen_foods,
            mut combat_stats,
        ) = data;

        targeted_eats.clear(); //TODO dirty, create a system specificaly to clear this.

        let mut turn_done: Vec<Entity> = Vec::new();
        /*
                //check if we managed to get a target
                for (entity, _animal, _pos, _carnivore, _cow, _turn) in (
                    &entities,
                    &animals,
                    &mut positions,
                    &carnivores,
                    &cows,
                    &turns,
                )
                    .join()
                {
                    if let Some(reached) = target_reacheds.get(entity) {
                        //TODO for now it eat directly I must add a fight
                        want_to_eats
                            .insert(
                                entity,
                                WantToEat {
                                    target: reached.target,
                                },
                            )
                            .expect("Unable to insert");

                        //if eat, end turn
                        turn_done.push(entity);

                    //TODO do not search a new target if the entity is already eating
                    } else {
                        //println!("no target reached");
                    }
                }
        */
        //check if we managed to get on our choosen food
        for (entity, _animal, pos, _carnivore, _cow, _turn, choosen_food) in (
            &entities,
            &animals,
            &positions,
            &carnivores,
            &cows,
            &turns,
            &my_choosen_foods,
        )
            .join()
        {
            //Since this stay up at the destruction of entity The entity ccan be destroyed an we need to check
            if let Some(food_pos) = positions.get(choosen_food.target) {
                //TODO for now it eat directly I must add a fight
                if in_contact(pos, food_pos) {
                    want_to_eats
                        .insert(
                            entity,
                            WantToEat {
                                target: choosen_food.target,
                            },
                        )
                        .expect("Unable to insert");

                    //if eat, end turn
                    turn_done.push(entity);
                }
            }
        }
        my_choosen_foods.clear();

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
        turn_done.clear();

        //Chose the food to go
        //first try to have his favorite food
        for (entity, viewshed, _animal, carnivore, cow, energy_reserve, _turn) in (
            &entities,
            &viewsheds,
            &animals,
            &carnivores,
            &cows,
            &energy_reserves,
            &turns,
        )
            .join()
        {
            //search for every possible food in the viewshed, and divide them acording to their categorie
            let mut found_leaf: Vec<Entity> = Vec::new();
            let mut found_other_specie: Vec<Entity> = Vec::new();
            let mut found_same_specie: Vec<Entity> = Vec::new();
            let my_specie = species.get(entity).unwrap();

            for visible_tile in viewshed.visible_tiles.iter() {
                let idx = map.xy_idx(visible_tile.x, visible_tile.y);
                for maybe_food in map.tile_content[idx].iter() {
                    if let Some(_leaf) = leafs.get(*maybe_food) {
                        found_leaf.push(*maybe_food);
                    }
                    if let Some(specie) = species.get(*maybe_food) {
                        if specie.name == my_specie.name {
                            found_same_specie.push(*maybe_food);
                        } else {
                            found_other_specie.push(*maybe_food);
                        }
                    }
                }
            }

            if energy_reserve.hunger == Hunger::Hungry {
                //Choose if the animal prefere to go for vegetable or meat
                //TODO  add hunger conditon before going for the non prefered food
                if cow.digestion > carnivore.digestion {
                    if choose_food(
                        found_leaf,
                        entity,
                        &mut positions,
                        &mut targeted_eats,
                        &mut go_targets,
                        &mut my_choosen_foods,
                        &mut combat_stats,
                    ) {
                        //if find food, end turn
                        turn_done.push(entity);
                    } else {
                        //TODO also use relative digestion between carnivore and cow
                        //if we didn't find food and if of reserve are compored to our capacity of digestion, then eat other food
                        if energy_reserve.get_relative_reserve() < carnivore.digestion {
                            if choose_food(
                                found_other_specie,
                                entity,
                                &mut positions,
                                &mut targeted_eats,
                                &mut go_targets,
                                &mut my_choosen_foods,
                                &mut combat_stats,
                            ) {
                                //if find food, end turn
                                turn_done.push(entity);
                            }
                        }
                    }
                } else {
                    if choose_food(
                        found_other_specie,
                        entity,
                        &mut positions,
                        &mut targeted_eats,
                        &mut go_targets,
                        &mut my_choosen_foods,
                        &mut combat_stats,
                    ) {
                        //if find food, end turn
                        turn_done.push(entity);
                    } else {
                        if energy_reserve.get_relative_reserve() < cow.digestion {
                            if choose_food(
                                found_leaf,
                                entity,
                                &mut positions,
                                &mut targeted_eats,
                                &mut go_targets,
                                &mut my_choosen_foods,
                                &mut combat_stats,
                            ) {
                                //if find food, end turn
                                turn_done.push(entity);
                            }
                        }
                    }
                }
            }
        }

        //check someone want to eat us
        //TODO make a better flee with real move using speed and go to.
        for (entity, _animal, _pos, _carnivore, _cow) in
            (&entities, &animals, &mut positions, &carnivores, &cows).join()
        {
            if let Some(targeted) = targeted_eats.get(entity) {
                //For now just flee if someone want to eat us
                let mut flee_list = Vec::new();
                flee_list.push(map.xy_idx(targeted.predator_pos.x, targeted.predator_pos.y) as i32);

                flees
                    .insert(entity, WantsToFlee { indices: flee_list })
                    .expect("Unable to insert");

                //if flee, end turn
                turn_done.push(entity);
            }
        }

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
        turn_done.clear();

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
        turn_done.clear();
    }
}

//In a list of possible food, choose the closest that is not taken by someone closer to the food
//return true if a food have been choosen
fn choose_food<'a>(
    found_foods: Vec<Entity>,
    entity: Entity,
    positions: &mut WriteStorage<'a, Position>,
    targeted_eats: &mut WriteStorage<'a, TargetedForEat>,
    go_targets: &mut WriteStorage<'a, GoOnTarget>,
    my_choosen_foods: &mut WriteStorage<'a, MyChoosenFood>,
    combat_stats: &mut WriteStorage<'a, CombatStats>,
) -> bool {
    let mut ret = false;
    let mut choosen_food: Option<Entity> = None;
    let mut min: f32 = std::f32::MAX;
    let pos = positions.get(entity).unwrap();

    for food in found_foods {
        let food_pos = positions.get(food).unwrap();
        let maybe_targeted_eat = targeted_eats.get(food);

        //If food can fight, only go if I am stronger
        if AmIStronger(&combat_stats, entity, food) {
            //if their is a other creature that want the target, then I only go if I am closer
            let mut competitor_distance = std::f32::MAX;
            if let Some(targeted) = maybe_targeted_eat {
                competitor_distance = targeted.distance;
            }
            let distance = rltk::DistanceAlg::Pythagoras
                .distance2d(Point::new(pos.x, pos.y), Point::new(food_pos.x, food_pos.y));
            if (distance < min) && (distance < competitor_distance) {
                choosen_food = Some(food);
                min = distance;
            }
        }
    }
    if let Some(food) = choosen_food {
        targeted_eats
            .insert(
                food,
                TargetedForEat {
                    //TODO add a flee if we are target to eat
                    predator: entity,
                    distance: min,
                    predator_pos: Point::new(pos.x, pos.y),
                },
            )
            .expect("Unable ot insert");
        go_targets
            .insert(
                entity,
                GoOnTarget {
                    target: food,
                    scope: SearchScope::Small,
                },
            )
            .expect("Unable to insert");
        my_choosen_foods
            .insert(entity, MyChoosenFood { target: food })
            .expect("Unable to insert");
        ret = true;
    }
    return ret;
}

pub fn in_contact(pos1: &Position, pos2: &Position) -> bool {
    let mut ret = false;
    if pos1.x >= pos2.x - 1 && pos1.x <= pos2.x + 1 {
        if pos1.y >= pos2.y - 1 && pos1.y <= pos2.y + 1 {
            ret = true;
        }
    }
    ret
}

//check combat stat to se if I am stronger
//Also return true If the enemy doesn't have combat stat
pub fn AmIStronger<'a>(
    combat_stats: &WriteStorage<'a, CombatStats>,
    me: Entity,
    enemy: Entity,
) -> bool {
    let my_stat = combat_stats.get(me).unwrap();
    let ret;

    if let Some(enemy_stat) = combat_stats.get(enemy) {
        if my_stat.power > enemy_stat.power {
            ret = true;
        } else {
            ret = false;
        }
    } else {
        ret = true
    }
    ret
}
