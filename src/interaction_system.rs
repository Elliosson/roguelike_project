extern crate specs;
use super::{
    raws::*, // See if we can not use them
    Interaction,
    Item,
    Name,
    Position,
    Renderable,
    ToDelete,
};

use rltk::RGB;
use specs::prelude::*;

pub struct InteractionSystem {}

//for now just destruct the interacted
impl<'a> System<'a> for InteractionSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        Entities<'a>,
        WriteExpect<'a, ObjectBuilder>,
        WriteExpect<'a, InteractionResquest>,
        WriteStorage<'a, ToDelete>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_entities, mut object_builder, mut interaction_request, mut to_deletes) = data;

        //parcours all interaction request
        for (x, y, interaction, interacted_entity) in &interaction_request.requests {
            //build object
            for to_build in &interaction.object_to_build {
                //ask for building the object
                object_builder.request(*x, *y, to_build.clone());
            }

            //eventualy destroy the entiety
            if interaction.destructif == true {
                to_deletes
                    .insert(*interacted_entity, ToDelete {})
                    .expect("Unable to insert delete entity");
            }
        }

        interaction_request.requests.clear();
    }
}

pub struct ObjectSpawnSystem {}

type ObjectSpawmerDataRef<'a, 'b> = (
    &'b Entities<'a>,
    &'b mut WriteStorage<'a, Position>,
    &'b mut WriteStorage<'a, Renderable>,
    &'b mut WriteStorage<'a, Name>,
    &'b mut WriteStorage<'a, Item>,
);

impl<'a> System<'a> for ObjectSpawnSystem {
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
        let (entities, mut positions, mut renderables, mut names, mut items, mut object_builder) =
            data;
        for request in object_builder.requests.iter() {
            //Get raw(json data) and build the object according to the json
            let raws: &RawMaster = &RAWS.lock().unwrap();
            spawn_named_item_ingame(
                (
                    &entities,
                    &mut positions, //TODO suppress the tuple
                    &mut renderables,
                    &mut names,
                    &mut items,
                ),
                raws,
                request.name.as_ref(),
                SpawnType::AtPosition {
                    x: request.x,
                    y: request.y,
                },
            )
        }

        object_builder.requests.clear();
    }
}

//almost duplicaton of rawmaster function , only diference is the insert because we don't have ecs
//TODO deplacer dans raw
//TODO Atention marker not added, opject will not be save
pub fn spawn_named_item_ingame(
    data: ObjectSpawmerDataRef,
    raws: &RawMaster,
    key: &str,
    pos: SpawnType,
) {
    let (entities, positions, renderables, names, items) = data;

    if raws.item_index.contains_key(key) {
        let item_template = &raws.raws.items[raws.item_index[key]];

        let new_entity = entities.create();

        // Spawn in the specified location
        match pos {
            SpawnType::AtPosition { x, y } => {
                positions
                    .insert(new_entity, Position { x, y })
                    .expect("Unable to inser position");
            }
        }

        // Renderable
        if let Some(renderable) = &item_template.renderable {
            renderables
                .insert(
                    new_entity,
                    Renderable {
                        glyph: rltk::to_cp437(renderable.glyph.chars().next().unwrap()),
                        fg: rltk::RGB::from_hex(&renderable.fg).expect("Invalid RGB"),
                        bg: rltk::RGB::from_hex(&renderable.bg).expect("Invalid RGB"),
                        render_order: renderable.order,
                    },
                )
                .expect("Unable to insert renderable");
        }

        names
            .insert(
                new_entity,
                Name {
                    name: item_template.name.clone(),
                },
            )
            .expect("Unable to insert name");

        items
            .insert(new_entity, Item {})
            .expect("Unable to insert item");
    } else {
        println!("Error: key: {} , is not know", key);
    }
}

// TODO add marker as in the classic builder
fn _wood_spawn<'a>(data: ObjectSpawmerDataRef, x: i32, y: i32) {
    let (entities, positions, renderables, names, items) = data;

    let p = entities.create();
    positions
        .insert(p, Position { x, y })
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

fn _apple_spawn<'a>(data: ObjectSpawmerDataRef, x: i32, y: i32) {
    let (entities, positions, renderables, names, items) = data;

    entities
        .build_entity()
        .with(Position { x, y }, positions)
        .with(
            Renderable {
                glyph: rltk::to_cp437('*'),
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
                render_order: 2,
            },
            renderables,
        )
        .with(
            Name {
                name: "Apple".to_string(),
            },
            names,
        )
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
        self.requests.push(ObjectRequest { x, y, name });
    }
}

pub struct InteractionResquest {
    requests: Vec<(i32, i32, Interaction, Entity)>,
}

impl InteractionResquest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> InteractionResquest {
        InteractionResquest {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x: i32, y: i32, interaction: Interaction, interacted_entity: Entity) {
        self.requests.push((x, y, interaction, interacted_entity));
    }
}
