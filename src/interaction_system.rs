extern crate specs;
use super::{
    gamelog::GameLog, AreaOfEffect, CombatStats, Confusion, Consumable, Equippable, Equipped,
    InBackpack, InflictsDamage, Item, Map, Name, Position, ProvidesHealing, Renderable,
    SufferDamage, WantsToDropItem, WantsToInteract, WantsToRemoveItem, WantsToUseItem,
};
use crate::spawner::wood;
use rltk::RGB;
use specs::prelude::*;

pub struct InteractionSystem {}

//for now just destruct the interacted
impl<'a> System<'a> for InteractionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, Entity>,
        WriteExpect<'a, GameLog>,
        WriteStorage<'a, WantsToInteract>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Name>,
        WriteExpect<'a, ObjectBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player_entity,
            mut gamelog,
            mut wants_interact,
            mut positions,
            names,
            mut wood_builder,
        ) = data;

        let mut to_remove: Vec<Entity> = Vec::new();

        for (entity, object, position) in (&entities, &wants_interact, &positions).join() {
            {
                let x = position.x;
                let y = position.y;

                //for now let wood on the floor, must find a way to specialise the interaction
                wood_builder.request(x, y);
            }

            to_remove.push(object.interacted);

            if object.interacted_by == *player_entity {
                gamelog.entries.insert(
                    0,
                    format!(
                        "You interact with the {}.",
                        names.get(object.interacted).unwrap().name
                    ),
                );
            }
        }

        for entity in to_remove {
            positions.remove(entity);
        }

        wants_interact.clear();
    }
}

pub struct WoodSpawnSystem {}

impl<'a> System<'a> for WoodSpawnSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, Name>,
        WriteStorage<'a, Item>,
        WriteExpect<'a, ObjectBuilder>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, mut positions, mut renderables, mut names, mut items, mut wood_builder) =
            data;
        for new_wood in wood_builder.requests.iter() {
            let p = entities.create();
            positions
                .insert(
                    p,
                    Position {
                        x: new_wood.x,
                        y: new_wood.y,
                    },
                )
                .expect("Unable to inser position");
            renderables
                .insert(
                    p,
                    Renderable {
                        glyph: rltk::to_cp437('*'),
                        fg: RGB::named(rltk::BURLYWOOD1),
                        bg: RGB::named(rltk::BLACK),
                        render_order: 2,
                    },
                )
                .expect("Unable to insert renderable");
            names
                .insert(
                    p,
                    Name {
                        name: "Wood".to_string(),
                    },
                )
                .expect("Unable to insert name");
            items.insert(p, Item {}).expect("Unable to insert item");
        }

        wood_builder.requests.clear();
    }
}

//TODO traveaux en cour pour object builder
struct ObjectRequest {
    x: i32,
    y: i32,
}

pub struct ObjectBuilder {
    requests: Vec<ObjectRequest>,
}

impl ObjectBuilder {
    #[allow(clippy::new_without_default)]
    pub fn new() -> ObjectBuilder {
        ObjectBuilder {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32) {
        self.requests.push(ObjectRequest { x, y });
    }
}
