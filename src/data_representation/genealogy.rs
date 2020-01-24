
/*use crate::{Date, Name, Position};
extern crate specs;
use crate::gamelog::GameLog;
use specs::prelude::*;
extern crate rltk;

pub struct BirthRegistery {
    borns: Vec<BirthCertificate>,
}

//on oublie pour l'instant, je pense que c'est plus malin de register les morts en faite
//et aussi je donne un composant BirthCertificate lors de la naissance pour qu'il conaisse tout de ses origine

//le faire lors de la mort plutot
impl BirthRegistery {
    #[allow(clippy::new_without_default)]
    pub fn new() -> BirthRegistery {
        BirthRegistery { borns: Vec::new() }
    }

    pub fn register(
        &mut self,
        name: Name,
        entity: Entity,
        parents: Vec<(Entity, Name)>,
        date: Date,
        position: Position,
    ) {
        self.borns.push(BirthCertificate {
            name,
            entity,
            parents,
            date,
            position,
        })
    }
}
/*
pub struct BirthCertificate {
    name: Name,
    entity: Entity,
    parents: Vec<(Entity, Name)>,
    date: Date,
    position: Position,
}

fn write_genealogy(ecs: &mut World) {
    let mut registery = ecs.write_resource::<BirthRegistery>();

    for certif in registery.borns.iter() {
        for parent in &certif.parents {
            println!("{} -> {}", parent.0.id(), certif.entity.id());
            println!("{} [label=\"{}]", certif.entity.id(), certif.name.name);
            println!("{} [label=\"{}]", parent.0.id(), parent.1.name);
        }
    }

    registery.borns.clear();
}
*/
