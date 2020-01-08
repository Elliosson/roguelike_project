extern crate specs;
use super::{
    Item, Name, Position, Renderable,
    Interaction, ToDelete
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
        let (
            _entities,
            mut object_builder,
            mut interaction_request,
            mut to_deletes
        ) = data;


        //parcours all interaction request
        for (x, y, interaction, interacted_entity) in &interaction_request.requests{
            //build object
            for to_build in &interaction.object_to_build{
                //ask for building the object
                object_builder.request(*x, *y, to_build.clone());
            }

            //eventualy destroy the entiety
            if interaction.destructif == true {
                to_deletes.insert(*interacted_entity, ToDelete{}).expect("Unable to insert delete entity");
            }
        }

        interaction_request.requests.clear();   
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



pub struct InteractionResquest{
    requests: Vec<(i32, i32, Interaction, Entity)>,
}

impl InteractionResquest {
    #[allow(clippy::new_without_default)]
    pub fn new() -> InteractionResquest {
        InteractionResquest {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, x:i32, y: i32, interaction: Interaction, interacted_entity: Entity) {
        self.requests.push((x, y, interaction, interacted_entity));
    }
}



