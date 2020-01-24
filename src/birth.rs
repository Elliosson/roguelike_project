extern crate specs;
use super::{raws::*, Date, EnergyReserve, Name, Position, SerializeMe, SoloReproduction};
use crate::gamelog::GameLog;
use crate::specs::saveload::{MarkedBuilder, SimpleMarker};
use specs::prelude::*;

#[derive(Clone)]
pub struct BirthCertificate {
    pub name: Name,
    pub entity: Entity,
    pub parents: Entity,
    pub date: Date,
    pub position: Position,
}

//for now just a few
#[derive(Clone)]
pub struct Mutations {
    pub solo_reproduction: Option<SoloReproduction>,
    pub energy_reserve: Option<EnergyReserve>,
}

#[derive(Clone)]
pub struct BirthRequest {
    pub certificate: BirthCertificate,
    pub mutations: Mutations,
}

//struc de demande de birth
//to insert in world
pub struct BirthRequetList {
    requests: Vec<BirthRequest>,
}

impl BirthRequetList {
    #[allow(clippy::new_without_default)]
    pub fn new() -> BirthRequetList {
        BirthRequetList {
            requests: Vec::new(),
        }
    }

    pub fn request(&mut self, certificate: BirthCertificate, mutations: Mutations) {
        self.requests.push(BirthRequest {
            certificate,
            mutations,
        });
    }
}

//registery of bith ever. to insert in world and to save in savegame
pub struct BirthRegistery {
    registery: Vec<BirthCertificate>,
}

impl BirthRegistery {
    #[allow(clippy::new_without_default)]
    pub fn new() -> BirthRegistery {
        BirthRegistery {
            registery: Vec::new(),
        }
    }

    pub fn insert(&mut self, certificate: BirthCertificate) {
        self.registery.push(certificate);
    }
}

pub fn give_birth(ecs: &mut World) {
    let mut birth_requests = ecs.write_resource::<BirthRequetList>().requests.clone();

    let mut birth_success: Vec<BirthCertificate> = Vec::new();

    // Using a scope to make the borrow checker happy
    {
        for birth_request in birth_requests.iter() {
            //appelle a la fonction creation entity avec raw
            if spawn_birth(ecs, birth_request.clone()) {
                birth_success.push(birth_request.certificate.clone());
            }
        }
    }

    let mut birth_requests_list = ecs.write_resource::<BirthRequetList>();
    birth_requests_list.requests.clear();

    let mut birth_registery = ecs.write_resource::<BirthRegistery>();
    for birth in birth_success {
        birth_registery.registery.push(birth);
    }
}

//TODO gerer les mutation ici ?
pub fn spawn_birth(ecs: &mut World, birth_request: BirthRequest) -> bool {
    //TODO appler la fonction specifique de creation d'une nouvelle creature avec heritage

    let mut ret = false;
    let key = &birth_request.certificate.name.name.clone();

    let raws: &RawMaster = &RAWS.lock().unwrap();
    if raws.prop_index.contains_key(key) {
        let spawn_result = spawn_born(
            raws,
            ecs.create_entity().marked::<SimpleMarker<SerializeMe>>(),
            birth_request,
        );
        if spawn_result.is_some() {
            ret = true;
        } else {
            println!("WARNING: We don't know how to spawn [{}]!", key);
        }
    } else {
        println!("WARNING: No keys {} !", key);
    }

    return ret;
}
