extern crate specs;
use super::{
    raws::*, Date, EnergyReserve, Name, Position, SerializeMe, SoloReproduction, UniqueId,
};
use crate::specs::saveload::{MarkedBuilder, SimpleMarker};
use specs::prelude::*;

#[derive(Clone)]
pub struct BirthCertificate {
    pub name: Name,
    pub entity: Entity,
    pub id: usize,
    pub parents: Entity,
    pub parent_id: usize,
    pub date: Date,
    pub position: Position,
}

#[derive(Clone)]
pub struct BirthForm {
    pub name: Name,
    pub parents: Entity,
    pub parent_id: usize,
    pub date: Date,
    pub position: Position,
}

//for now just a few
#[derive(Clone)]
pub struct Mutations {
    pub solo_reproduction: Option<SoloReproduction>,
    pub energy_reserve: Option<EnergyReserve>,
}

impl Mutations {
    pub fn new() -> Mutations {
        Mutations {
            solo_reproduction: None,
            energy_reserve: None,
        }
    }
}

#[derive(Clone)]
pub struct BirthRequest {
    pub form: BirthForm,
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

    pub fn request(&mut self, form: BirthForm, mutations: Mutations) {
        self.requests.push(BirthRequest { form, mutations });
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

    pub fn get(&self) -> Vec<BirthCertificate> {
        self.registery.clone()
    }
}

// Spawn the birth request and create the birth certificate if success
pub fn give_birth(ecs: &mut World) {
    let birth_requests = ecs.write_resource::<BirthRequetList>().requests.clone();

    let mut birth_success: Vec<(Entity, BirthForm)> = Vec::new();

    // Create the entity
    {
        for birth_request in birth_requests.iter() {
            //appelle a la fonction creation entity avec raw
            let entity_builder = ecs.create_entity().marked::<SimpleMarker<SerializeMe>>();

            if let Some(spawn_result) = spawn_birth(entity_builder, birth_request.clone()) {
                birth_success.push((spawn_result, birth_request.form.clone()));
            }
        }
    }

    {
        let mut birth_requests_list = ecs.write_resource::<BirthRequetList>();
        birth_requests_list.requests.clear();
    }

    //Create Birth certificate
    {
        let mut birth_registery = ecs.write_resource::<BirthRegistery>();
        let unique_ids = ecs.read_storage::<UniqueId>();
        for (entity, form) in birth_success {
            let certif = BirthCertificate {
                name: form.name,
                entity: entity,
                id: unique_ids
                    .get(entity)
                    .expect("Error: No uniqueId in the new born entity")
                    .get(),
                parents: form.parents,
                parent_id: form.parent_id,
                date: form.date,
                position: form.position,
            };
            birth_registery.insert(certif);
        }
    }
}

//TODO gerer les mutation ici ?
pub fn spawn_birth(entity: EntityBuilder, birth_request: BirthRequest) -> Option<Entity> {
    //TODO appler la fonction specifique de creation d'une nouvelle creature avec heritage

    let mut spawn_result = None;

    let key = &birth_request.form.name.name.clone();

    let raws: &RawMaster = &RAWS.lock().unwrap();
    if raws.prop_index.contains_key(key) {
        spawn_result = spawn_born(raws, entity, birth_request);
        if spawn_result.is_some() {

        } else {
            println!("WARNING: We don't know how to spawn [{}]!", key);
        }
    } else {
        println!("WARNING: No keys {} !", key);
    }

    return spawn_result;
}
