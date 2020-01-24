use crate::BirthRegistery;
extern crate specs;
use specs::prelude::*;
extern crate rltk;

pub fn write_genealogy(ecs: &mut World) {
    let registery = ecs.fetch::<BirthRegistery>();

    for certif in registery.get().iter() {
        println!("{} -> {}", certif.parents.id(), certif.entity.id());
        println!("{} [label=\"{}]", certif.entity.id(), certif.name.name);
        println!("{} [label=\"{}]", certif.parents.id(), certif.name.name);
    }
}
