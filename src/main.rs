extern crate rltk;
extern crate serde;
use rltk::{Console, GameState, Point, Rltk};
extern crate specs;
use specs::prelude::*;
use specs::saveload::{SimpleMarker, SimpleMarkerAllocator};
#[macro_use]
extern crate specs_derive;
mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
use visibility_system::VisibilitySystem;
mod monster_ai_system;
use monster_ai_system::MonsterAI;
mod map_indexing_system;
use map_indexing_system::MapIndexingSystem;
mod melee_combat_system;
use melee_combat_system::MeleeCombatSystem;
mod damage_system;
use damage_system::DamageSystem;
mod gamelog;
mod gui;
mod spawner;
use spawner::*;
mod inventory_system;
use inventory_system::{ItemCollectionSystem, ItemDropSystem, ItemRemoveSystem, ItemUseSystem};
mod movement_system;
mod object_deleter;
pub mod random_table;
pub mod raws;
pub mod saveload_system;
use movement_system::MovementSystem;
pub mod ai;
use ai::*;
mod tiletype;
mod trigger_system;
use tiletype::{tile_walkable, TileType};
pub mod systems;
use systems::*;
mod algo;
mod birth;
use birth::{BirthForm, BirthRegistery, BirthRequetList, Mutations};
mod atomic_funtions;
mod data_representation;

#[macro_use]
extern crate lazy_static;

rltk::add_wasm_support!();

pub const WINDOWWIDTH: usize = 200;
pub const WINDOWHEIGHT: usize = 90;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    AwaitingInput,
    PreRun,
    PlayerTurn,
    MonsterTurn,
    ShowInventory,
    ShowDropItem,
    ShowTargeting {
        range: i32,
        item: Entity,
    },
    MainMenu {
        menu_selection: gui::MainMenuSelection,
    },
    SaveGame,
    NextLevel,
    ShowRemoveItem,
    GameOver,
    ObjectInteraction,
    TemperatureMap,
}

pub struct State {
    pub ecs: World,
}

impl State {
    fn run_systems(&mut self) {
        let mut date = DateSystem {};
        date.run_now(&self.ecs);
        let mut temperature = TemperatureSystem {};
        temperature.run_now(&self.ecs);
        let mut humidity = HumiditySystem {};
        humidity.run_now(&self.ecs);
        let mut temperature_sens = TemperatureSensitivitySystem {};
        temperature_sens.run_now(&self.ecs);
        let mut humidity_sens = HumiditySensitivitySystem {};
        humidity_sens.run_now(&self.ecs);
        let mut specie = SpecieSystem {};
        specie.run_now(&self.ecs);
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
        let mut cow = CowAI {};
        cow.run_now(&self.ecs);
        let mut carnivore_ai = CarnivorousAI {};
        carnivore_ai.run_now(&self.ecs);
        let mut flee_ai = FleeAI {};
        flee_ai.run_now(&self.ecs);
        let mut mapindex = MapIndexingSystem {};
        mapindex.run_now(&self.ecs);
        let mut melee = MeleeCombatSystem {};
        melee.run_now(&self.ecs);
        let mut damage = DamageSystem {};
        damage.run_now(&self.ecs);
        let mut pickup = ItemCollectionSystem {};
        pickup.run_now(&self.ecs);
        let mut itemuse = ItemUseSystem {};
        itemuse.run_now(&self.ecs);
        let mut drop_items = ItemDropSystem {};
        drop_items.run_now(&self.ecs);
        let mut item_remove = ItemRemoveSystem {};
        item_remove.run_now(&self.ecs);
        let mut object_spawn = ObjectSpawnSystem {};
        object_spawn.run_now(&self.ecs);
        let mut interaction = InteractionSystem {};
        interaction.run_now(&self.ecs);
        let mut go_target = GoTargetSystem {};
        go_target.run_now(&self.ecs);
        let mut movement = MovementSystem {};
        movement.run_now(&self.ecs);
        let mut eating = EatingSystem {};
        eating.run_now(&self.ecs);
        let mut veg_grow = VegetableGrowSystem {};
        veg_grow.run_now(&self.ecs);
        let mut energy = EnergySystem {};
        energy.run_now(&self.ecs);
        //let mut solo_reprod = SoloReproductionSystem {};
        //solo_reprod.run_now(&self.ecs);
        let mut gendered_reprod = GenderedReproductionSystem {};
        gendered_reprod.run_now(&self.ecs);
        let mut prop_spawmer = PropSpawnerSystem {};
        prop_spawmer.run_now(&self.ecs);
        let mut aging = AgingSystem {};
        aging.run_now(&self.ecs);
        let mut named_counter = NamedCounterSystem {};
        named_counter.run_now(&self.ecs);
        let mut stat = StatSystem {};
        stat.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        let mut newrunstate;
        {
            let runstate = self.ecs.fetch::<RunState>();
            newrunstate = *runstate;
        }

        ctx.cls();

        match newrunstate {
            RunState::MainMenu { .. } => {}
            RunState::GameOver { .. } => {}
            _ => {
                draw_map(&self.ecs, ctx);

                {
                    let positions = self.ecs.read_storage::<Position>();
                    let renderables = self.ecs.read_storage::<Renderable>();

                    let mut data = (&positions, &renderables).join().collect::<Vec<_>>();
                    data.sort_by(|&a, &b| b.1.render_order.cmp(&a.1.render_order));
                    for (pos, render) in data.iter() {
                        ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
                    }

                    gui::draw_ui(&self.ecs, ctx);
                }
            }
        }

        match newrunstate {
            RunState::PreRun => {
                //self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::AwaitingInput => {
                newrunstate = player_input(self, ctx);
            }
            RunState::PlayerTurn => {
                self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::MonsterTurn;
            }
            RunState::MonsterTurn => {
                //self.run_systems();
                self.ecs.maintain();
                newrunstate = RunState::AwaitingInput;
            }
            RunState::ShowInventory => {
                let result = gui::show_inventory(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let is_ranged = self.ecs.read_storage::<Ranged>();
                        let is_item_ranged = is_ranged.get(item_entity);
                        if let Some(is_item_ranged) = is_item_ranged {
                            newrunstate = RunState::ShowTargeting {
                                range: is_item_ranged.range,
                                item: item_entity,
                            };
                        } else {
                            let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                            intent
                                .insert(
                                    *self.ecs.fetch::<Entity>(),
                                    WantsToUseItem {
                                        item: item_entity,
                                        target: None,
                                    },
                                )
                                .expect("Unable to insert intent");
                            newrunstate = RunState::PlayerTurn;
                        }
                    }
                }
            }

            RunState::ObjectInteraction => {
                let result = gui::show_object_interaction_choice(self, ctx);
                match result.0 {
                    gui::InteractionMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::InteractionMenuResult::NoResponse => {}
                    gui::InteractionMenuResult::Selected => {
                        // TODO provisory just ask for the creation of an object of the name
                        let interaction_tuple = result.1.unwrap();
                        let (x, y, interaction, interacted_entity) = interaction_tuple;

                        let mut interaction_requests =
                            self.ecs.write_resource::<InteractionResquest>();
                        interaction_requests.request(x, y, interaction.clone(), interacted_entity);

                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowDropItem => {
                let result = gui::drop_item_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToDropItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToDropItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowRemoveItem => {
                let result = gui::remove_item_menu(self, ctx);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let item_entity = result.1.unwrap();
                        let mut intent = self.ecs.write_storage::<WantsToRemoveItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToRemoveItem { item: item_entity },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::ShowTargeting { range, item } => {
                let result = gui::ranged_target(self, ctx, range);
                match result.0 {
                    gui::ItemMenuResult::Cancel => newrunstate = RunState::AwaitingInput,
                    gui::ItemMenuResult::NoResponse => {}
                    gui::ItemMenuResult::Selected => {
                        let mut intent = self.ecs.write_storage::<WantsToUseItem>();
                        intent
                            .insert(
                                *self.ecs.fetch::<Entity>(),
                                WantsToUseItem {
                                    item,
                                    target: result.1,
                                },
                            )
                            .expect("Unable to insert intent");
                        newrunstate = RunState::PlayerTurn;
                    }
                }
            }
            RunState::MainMenu { .. } => {
                let result = gui::main_menu(self, ctx);
                match result {
                    gui::MainMenuResult::NoSelection { selected } => {
                        newrunstate = RunState::MainMenu {
                            menu_selection: selected,
                        }
                    }
                    gui::MainMenuResult::Selected { selected } => match selected {
                        gui::MainMenuSelection::NewGame => newrunstate = RunState::PreRun,
                        gui::MainMenuSelection::LoadGame => {
                            saveload_system::load_game(&mut self.ecs);
                            newrunstate = RunState::AwaitingInput;
                            saveload_system::delete_save();
                        }
                        gui::MainMenuSelection::Quit => {
                            ::std::process::exit(0);
                        }
                    },
                }
            }
            RunState::GameOver => {
                let result = gui::game_over(ctx);
                match result {
                    gui::GameOverResult::NoSelection => {}
                    gui::GameOverResult::QuitToMenu => {
                        self.game_over_cleanup();
                        newrunstate = RunState::MainMenu {
                            menu_selection: gui::MainMenuSelection::NewGame,
                        };
                    }
                }
            }
            RunState::SaveGame => {
                saveload_system::save_game(&mut self.ecs);
                data_representation::world_state_log(&mut self.ecs).unwrap(); //TODO it's fuck with the log is i  write it's here, but it's better for performance
                data_representation::write_genealogy(&mut self.ecs).unwrap();
                newrunstate = RunState::MainMenu {
                    menu_selection: gui::MainMenuSelection::LoadGame,
                };
            }
            RunState::NextLevel => {
                self.goto_next_level();
                newrunstate = RunState::PreRun;
            }
            RunState::TemperatureMap => {
                let result = gui::temperature_map(self, ctx);
                match result {
                    gui::TemperatureMapResult::Cancel => newrunstate = RunState::PlayerTurn,
                    gui::TemperatureMapResult::NoResponse => {}
                }
            }
        }

        {
            let mut runwriter = self.ecs.write_resource::<RunState>();
            *runwriter = newrunstate;
        }
        damage_system::delete_the_dead(&mut self.ecs);
        birth::give_birth(&mut self.ecs);
        object_deleter::delete_entity_to_delete(&mut self.ecs);
    }
}

impl State {
    fn entities_to_remove_on_level_change(&mut self) -> Vec<Entity> {
        let entities = self.ecs.entities();
        let player = self.ecs.read_storage::<Player>();
        let backpack = self.ecs.read_storage::<InBackpack>();
        let player_entity = self.ecs.fetch::<Entity>();
        let equipped = self.ecs.read_storage::<Equipped>();

        let mut to_delete: Vec<Entity> = Vec::new();
        for entity in entities.join() {
            let mut should_delete = true;

            // Don't delete the player
            let p = player.get(entity);
            if let Some(_p) = p {
                should_delete = false;
            }

            // Don't delete the player's equipment
            let bp = backpack.get(entity);
            if let Some(bp) = bp {
                if bp.owner == *player_entity {
                    should_delete = false;
                }
            }

            let eq = equipped.get(entity);
            if let Some(eq) = eq {
                if eq.owner == *player_entity {
                    should_delete = false;
                }
            }

            if should_delete {
                to_delete.push(entity);
            }
        }

        to_delete
    }

    fn goto_next_level(&mut self) {
        // Delete entities that aren't the player or his/her equipment
        let to_delete = self.entities_to_remove_on_level_change();
        for target in to_delete {
            self.ecs
                .delete_entity(target)
                .expect("Unable to delete entity");
        }

        // Interactable a new map and place the player
        let worldmap;
        let current_depth;
        {
            let mut worldmap_resource = self.ecs.write_resource::<Map>();
            current_depth = worldmap_resource.depth;
            *worldmap_resource = Map::new_map_rooms_and_corridors(current_depth + 1);
            worldmap = worldmap_resource.clone();
        }

        // Spawn bad guys
        for room in worldmap.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.ecs, room, current_depth + 1);
        }

        // Place the player and update resources
        let (player_x, player_y) = worldmap.rooms[0].center();
        let mut player_position = self.ecs.write_resource::<Point>();
        *player_position = Point::new(player_x, player_y);
        let mut position_components = self.ecs.write_storage::<Position>();
        let player_entity = self.ecs.fetch::<Entity>();
        let player_pos_comp = position_components.get_mut(*player_entity);
        if let Some(player_pos_comp) = player_pos_comp {
            player_pos_comp.x = player_x;
            player_pos_comp.y = player_y;
        }

        // Mark the player's visibility as dirty
        let mut viewshed_components = self.ecs.write_storage::<Viewshed>();
        let vs = viewshed_components.get_mut(*player_entity);
        if let Some(vs) = vs {
            vs.dirty = true;
        }

        // Notify the player and give them some health
        let mut gamelog = self.ecs.fetch_mut::<gamelog::GameLog>();
        gamelog.entries.insert(
            0,
            "You descend to the next level, and take a moment to heal.".to_string(),
        );
        let mut player_health_store = self.ecs.write_storage::<CombatStats>();
        let player_health = player_health_store.get_mut(*player_entity);
        if let Some(player_health) = player_health {
            player_health.hp = i32::max(player_health.hp, player_health.max_hp / 2);
        }
    }

    fn game_over_cleanup(&mut self) {
        // Delete everything
        let mut to_delete = Vec::new();
        for e in self.ecs.entities().join() {
            to_delete.push(e);
        }
        for del in to_delete.iter() {
            self.ecs.delete_entity(*del).expect("Deletion failed");
        }

        // Interactable a new map and place the player
        let worldmap;
        {
            let mut worldmap_resource = self.ecs.write_resource::<Map>();
            *worldmap_resource = Map::new_map_rooms_and_corridors(1);
            worldmap = worldmap_resource.clone();
        }

        // Spawn bad guys
        for room in worldmap.rooms.iter().skip(1) {
            spawner::spawn_room(&mut self.ecs, room, 1);
        }

        // Place the player and update resources
        let (player_x, player_y) = worldmap.rooms[0].center();
        let player_entity = spawner::player(&mut self.ecs, player_x, player_y);
        let mut player_position = self.ecs.write_resource::<Point>();
        *player_position = Point::new(player_x, player_y);
        let mut position_components = self.ecs.write_storage::<Position>();
        let mut player_entity_writer = self.ecs.write_resource::<Entity>();
        *player_entity_writer = player_entity;
        let player_pos_comp = position_components.get_mut(player_entity);
        if let Some(player_pos_comp) = player_pos_comp {
            player_pos_comp.x = player_x;
            player_pos_comp.y = player_y;
        }

        // Mark the player's visibility as dirty
        let mut viewshed_components = self.ecs.write_storage::<Viewshed>();
        let vs = viewshed_components.get_mut(player_entity);
        if let Some(vs) = vs {
            vs.dirty = true;
        }
    }
}

fn main() {
    let mut context = Rltk::init_simple8x8(
        WINDOWWIDTH as u32,
        WINDOWHEIGHT as u32,
        "Ecosystem simulator",
        "resources",
    );
    context.with_post_scanlines(true);
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<WantsToMelee>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<Item>();
    gs.ecs.register::<ProvidesHealing>();
    gs.ecs.register::<InflictsDamage>();
    gs.ecs.register::<AreaOfEffect>();
    gs.ecs.register::<Consumable>();
    gs.ecs.register::<Ranged>();
    gs.ecs.register::<InBackpack>();
    gs.ecs.register::<WantsToPickupItem>();
    gs.ecs.register::<WantsToUseItem>();
    gs.ecs.register::<WantsToDropItem>();
    gs.ecs.register::<Confusion>();
    gs.ecs.register::<SimpleMarker<SerializeMe>>();
    gs.ecs.register::<SerializationHelper>();
    gs.ecs.register::<Equippable>();
    gs.ecs.register::<Equipped>();
    gs.ecs.register::<MeleePowerBonus>();
    gs.ecs.register::<DefenseBonus>();
    gs.ecs.register::<WantsToRemoveItem>();
    gs.ecs.register::<Interactable>();
    gs.ecs.register::<InteractableObject>();
    gs.ecs.register::<WantsToInteract>();
    gs.ecs.register::<Interaction>();
    gs.ecs.register::<ToDelete>();
    gs.ecs.register::<Cow>();
    gs.ecs.register::<Leaf>();
    gs.ecs.register::<WantToEat>();
    gs.ecs.register::<ApplyMove>();
    gs.ecs.register::<ApplyTeleport>();
    gs.ecs.register::<MyTurn>();
    gs.ecs.register::<MoveMode>();
    gs.ecs.register::<EntityMoved>();
    gs.ecs.register::<EntryTrigger>();
    gs.ecs.register::<Tree>();
    gs.ecs.register::<EnergyReserve>();
    gs.ecs.register::<SoloReproduction>();
    gs.ecs.register::<WantsToDuplicate>();
    gs.ecs.register::<UniqueId>();
    gs.ecs.register::<Aging>();
    gs.ecs.register::<InUse>();
    gs.ecs.register::<TemperatureSensitive>();
    gs.ecs.register::<Specie>();
    gs.ecs.register::<HumiditySensitive>();
    gs.ecs.register::<Speed>();
    gs.ecs.register::<GoOnTarget>();
    gs.ecs.register::<TargetReached>();
    gs.ecs.register::<Carnivore>();
    gs.ecs.register::<WantsToFlee>();
    gs.ecs.register::<TargetedForEat>();
    gs.ecs.register::<Animal>();

    gs.ecs.insert(SimpleMarkerAllocator::<SerializeMe>::new());

    raws::load_raws();

    let map: Map = Map::new_map();
    let (player_x, player_y) = map.rooms[0].center();

    let player_entity = spawner::player(&mut gs.ecs, player_x, player_y);

    gs.ecs.insert(rltk::RandomNumberGenerator::new());
    for room in map.rooms.iter() {
        spawner::spawn_trees(&mut gs.ecs, room);

        spawn_named(&mut gs.ecs, "Cow", 30, 2);
        spawn_named(&mut gs.ecs, "Cow", 30, 3);
        spawn_named(&mut gs.ecs, "Cow", 30, 4);
        spawn_named(&mut gs.ecs, "Cow", 30, 5);
        spawn_named(&mut gs.ecs, "Cow", 30, 6);
        spawn_named(&mut gs.ecs, "Cow", 30, 7);
        spawn_named(&mut gs.ecs, "Cow", 30, 8);
        spawn_named(&mut gs.ecs, "Cow", 30, 9);
        spawn_named(&mut gs.ecs, "Cow", 30, 10);
        spawn_named(&mut gs.ecs, "Cow", 30, 11);
        spawn_named(&mut gs.ecs, "Cow", 30, 12);
        spawn_named(&mut gs.ecs, "Cow", 30, 13);
        spawn_named(&mut gs.ecs, "Cow", 30, 14);
        //spawn_named(&mut gs.ecs, "Wolve", 20, 10);
        //spawn_named(&mut gs.ecs, "Wolve", 20, 11);
    }

    gs.ecs.insert(map);
    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(player_entity);
    gs.ecs.insert(RunState::MainMenu {
        menu_selection: gui::MainMenuSelection::NewGame,
    });
    gs.ecs.insert(gamelog::GameLog {
        entries: vec!["Welcome to Rusty Roguelike".to_string()],
    });
    gs.ecs.insert(ObjectBuilder::new());
    gs.ecs.insert(InteractionResquest::new());
    gs.ecs.insert(Date::new());
    gs.ecs.insert(BirthRequetList::new());
    gs.ecs.insert(BirthRegistery::new());
    gs.ecs.insert(gamelog::WorldStatLog {
        entries: vec!["Rust Roguelike log file".to_string()],
    });
    gs.ecs
        .insert(gamelog::SpeciesInstantLog { entries: vec![] });

    rltk::main_loop(context, gs);
}
