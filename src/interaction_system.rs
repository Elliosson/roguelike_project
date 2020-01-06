extern crate specs;
use super::{
    gamelog::GameLog, AreaOfEffect, CombatStats, Confusion, Consumable, Equippable, Equipped,
    InBackpack, InflictsDamage, Item, Map, Name, Position, ProvidesHealing, Renderable,
    SufferDamage, WantsToDropItem, WantsToInteract, WantsToRemoveItem, WantsToUseItem, SerializeMe,
    InteractableObject
};
use crate::spawner::wood;
use rltk::RGB;
use specs::prelude::*;

pub struct InteractionSystem {}

use crate::specs::saveload::{MarkedBuilder, SimpleMarker};

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
        ReadStorage<'a, InteractableObject>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            player_entity,
            mut gamelog,
            mut wants_interacts,
            mut positions,
            names,
            mut object_builder,
            interactable_object,
        ) = data;

        let mut to_remove: Vec<Entity> = Vec::new();

        for (entity, want_interact, position) in (&entities, &wants_interacts, &positions).join() {
            {
                let x = position.x;
                let y = position.y;

                let interactions = interactable_object.get(want_interact.interacted);

                match interactions{
                    None =>{}
                    //fo now just take the first string and construct the object in the first string 
                    Some(possibles_interactions) =>{
                        if possibles_interactions.interactions.len() > 0{
                            //create an object wiht all the interaction to send to the gui.interactions
                            //Le plus simple c'est de les push dans un object dedier à cette effet en faite
                        }

                        //provisoir
                        for interaction in &possibles_interactions.interactions{
                            let name = interaction.clone();
                            object_builder.request(x, y, name);

                        }

                    }
                }

                //for now let wood on the floor, must find a way to specialise the interaction
                //object_builder.request(x, y, "Apple".to_string());
            }

            to_remove.push(want_interact.interacted);

            if want_interact.interacted_by == *player_entity {
                gamelog.entries.insert(
                    0,
                    format!(
                        "You interact with the {}.",
                        names.get(want_interact.interacted).unwrap().name
                    ),
                );
            }
        }

        for entity in to_remove {
            positions.remove(entity);
        }

        wants_interacts.clear();
    }
}

pub struct WoodSpawnSystem {}

type ObjectSpawmerDataRef<'a, 'b> = (
    &'b Entities<'a>,
    &'b mut WriteStorage<'a, Position>,
    &'b mut WriteStorage<'a, Renderable>,
    &'b mut WriteStorage<'a, Name>,
    &'b mut WriteStorage<'a, Item>,
);

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
        for request in wood_builder.requests.iter() {
            match request.name.as_ref() {
                "Wood" => wood_spawn((&entities, &mut positions, &mut renderables, &mut names, &mut items), request.x, request.y),
                "Apple" => apple_spawn((&entities, &mut positions, &mut renderables, &mut names, &mut items), request.x, request.y),
                _ => { println!("Error: objectBuilder do noet know this object")},
            }
        }

        wood_builder.requests.clear();
    
    }


}


// TODO add marker as in the classic builder
fn wood_spawn<'a>(data: ObjectSpawmerDataRef, x: i32, y: i32)
{
    let (entities,  positions,  renderables,  names,  items) =
    data;

    let p = entities.create();
    positions
        .insert(
            p,
            Position {
                x,
                y,
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



fn apple_spawn<'a>(data: ObjectSpawmerDataRef, x: i32, y: i32)
{
    let (entities,  positions,  renderables,  names,  items) =
    data;

    entities.build_entity()
        .with(Position { x, y }, positions)
        .with(Renderable {
            glyph: rltk::to_cp437('*'),
            fg: RGB::named(rltk::RED),
            bg: RGB::named(rltk::BLACK),
            render_order: 2,
        }, renderables)
        .with(Name {
            name: "Apple".to_string(),
        }, names)
        .with(Item {}, items)
        .build();
}

//TODO traveaux en cour pour object builder
struct ObjectRequest {
    x: i32,
    y: i32,
    name: String,
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

    pub fn request(&mut self, x: i32, y: i32, name: String) {
        self.requests.push(ObjectRequest { x, y, name});
    }
}



