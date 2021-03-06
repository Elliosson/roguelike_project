extern crate specs;
use super::{
    algo, raws,
    raws::{RawMaster, RAWS},
    Carnivore, CombatStats, Date, EnergyReserve, Herbivore, HumiditySensitive, Name, Position,
    Renderable, Reproduction, SerializeMe, Specie, Speed, TemperatureSensitive, UniqueId,
};
use crate::specs::saveload::{MarkedBuilder, SimpleMarker};

use rand::Rng;
use specs::prelude::*;
use std::cmp::{max, min};

#[derive(Clone)]
pub struct BirthCertificate {
    pub name: Name,
    pub entity: Entity,
    pub id: usize,
    pub parent: Entity,
    pub parent_id: usize,
    pub male_parent: Option<Entity>,
    pub male_parent_id: Option<usize>,
    pub date: Date,
    pub position: Position,
}

#[derive(Clone)]
pub struct BirthForm {
    pub name: Name,
    pub parent: Entity,
    pub parent_id: usize,
    pub male_parent: Option<Entity>,
    pub male_parent_id: Option<usize>,
    pub date: Date,
    pub position: Position,
}

//for now just a few
#[derive(Clone)]
pub struct Mutations {
    pub reproduction: Option<Reproduction>,
    pub energy_reserve: Option<EnergyReserve>,
    pub temp_sensi: Option<TemperatureSensitive>,
    pub hum_sensi: Option<HumiditySensitive>,
    pub specie: Option<Specie>,
    pub renderable: Option<Renderable>,
    pub speed: Option<Speed>,
    pub herbivore: Option<Herbivore>,
    pub carnivore: Option<Carnivore>,
    pub combat_stat: Option<CombatStats>,
}

impl Mutations {
    pub fn new() -> Mutations {
        Mutations {
            reproduction: None,
            energy_reserve: None,
            temp_sensi: None,
            hum_sensi: None,
            specie: None,
            renderable: None,
            speed: None,
            herbivore: None,
            carnivore: None,
            combat_stat: None,
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
                parent: form.parent,
                parent_id: form.parent_id,
                male_parent: form.male_parent,
                male_parent_id: form.male_parent_id,
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
        spawn_result = raws::spawn_born(
            raws,
            entity,
            birth_request.form,
            change_mutation(birth_request.mutations),
        );
        if spawn_result.is_some() {
            //println!("A new entity is born");
        } else {
            println!("WARNING: We don't know how to spawn [{}]!", key);
        }
    } else {
        println!("WARNING: No keys {} !", key);
    }

    return spawn_result;
}

//Take an already existing set of mutation randomly add some new mutatition
//TODO create a new mutation to avois transmission of thing like energy reserve and hunger
pub fn change_mutation(mut mutations: Mutations) -> Mutations {
    let mut rng = rand::thread_rng();

    //intit todo suppress
    let mut birth_energy = 50.0;

    //For now just change the parametere of soloreprod
    if let Some(reprod) = &mut mutations.reproduction {
        birth_energy = reprod.birth_energy as f32;

        //reprod.cost += rng.gen_range(-1, 2);
        reprod.offset_threshold =
            algo::add_or_zero(reprod.offset_threshold, rng.gen_range(-10, 11));
        reprod.birth_energy = algo::add_or_zero(reprod.birth_energy, rng.gen_range(-10, 11));
    }

    if let Some(energy_res) = &mut mutations.energy_reserve {
        energy_res.max_reserve += rng.gen_range(-10, 11) as f32;

        //Set the birth energy here problably not the best place
        energy_res.reserve = birth_energy;
    }

    let new_comsuption = base_comsumption(mutations.clone());

    if let Some(energy_res) = &mut mutations.energy_reserve {
        energy_res.base_consumption = new_comsuption;
    }

    if let Some(temp_sensi) = &mut mutations.temp_sensi {
        temp_sensi.optimum += rng.gen_range(-2, 3) as f32;
    }

    if let Some(hum_sensi) = &mut mutations.hum_sensi {
        hum_sensi.optimum += rng.gen_range(-4.0, 4.0);
    }

    if let Some(speed) = &mut mutations.speed {
        speed.base_point_per_turn = min(
            100,
            max(0, speed.base_point_per_turn + rng.gen_range(-3, 4)),
        );
        speed.move_point = 0;
    }

    if let Some(herbivore) = &mut mutations.herbivore {
        herbivore.digestion = f32::min(
            1.0,
            f32::max(
                0.0,
                herbivore.digestion + (rng.gen_range(-1, 2) as f32 / 100.0),
            ),
        );
    }

    if let Some(carnivore) = &mut mutations.carnivore {
        carnivore.digestion = f32::min(
            1.0,
            f32::max(
                0.0,
                carnivore.digestion + (rng.gen_range(-1, 2) as f32 / 100.0),
            ),
        );
    }

    if let Some(combat_stat) = &mut mutations.combat_stat {
        combat_stat.power = i32::max(0, combat_stat.power + (rng.gen_range(-2, 3)));
    }
    mutations
}

//TODO ajouter des poid pour moderer les facteurs entre eux
fn base_comsumption(mutations: Mutations) -> f32 {
    let mut features_cost: f32 = 0.0;

    if let Some(_reprod) = &mutations.reproduction {
        //features_cost += reprod.cost as f32;
        //features_cost += reprod.threshold as f32;
    }

    if let Some(energy_res) = &mutations.energy_reserve {
        features_cost += energy_res.max_reserve / 200.0;
    }

    if let Some(speed) = &mutations.speed {
        features_cost += (speed.base_point_per_turn * speed.base_point_per_turn) as f32 / 300.0;
        //features_cost += speed.base_point_per_turn as f32 / 100.0;
    }

    if let Some(herbivore) = &mutations.herbivore {
        features_cost += herbivore.digestion * 4.0;
    }

    if let Some(carnivore) = &mutations.carnivore {
        features_cost += carnivore.digestion * 4.0;
    }

    //multiply the 2 to discourage muliple specialisation
    if let Some(carnivore) = &mutations.carnivore {
        if let Some(herbivore) = &mutations.carnivore {
            features_cost += carnivore.digestion * herbivore.digestion * 8.0;
        }
    }

    if let Some(combat_stat) = &mutations.combat_stat {
        features_cost += combat_stat.power as f32 / 100.0;
    }
    let new_consuption: f32 = features_cost / 3.0;
    new_consuption
}

#[cfg(test)]
mod tests {
    /*
    use super::*;
    use crate::Hunger;
    //Hard to test random
    #[test]
    fn change_mutation_test() {
        let mutations = Mutations {
            reproduction: Some(Reproduction {
                threshold: 101,
                cost: 102,
            }),
            energy_reserve: Some(EnergyReserve {
                reserve: 103.0,
                max_reserve: 104.0,
                base_consumption: 105.0,
                hunger: Hunger::Full,
            }),
        };

        let new_mut = change_mutation(mutations);

        //Pretty hard to test random
        //assert_ne!(new_mut.reproduction.unwrap().threshold, 101);
    }
    */
}
