use super::{RawEnergyReserve, RawViewshed};
use crate::components::*;
use serde::Deserialize;
use std::collections::HashMap; //todo maybe not ok

#[derive(Deserialize, Debug)]
pub struct Prop {
    pub name: String,
    pub renderable: Option<super::item_structs::Renderable>,
    pub hidden: Option<bool>,
    pub blocks_tile: Option<bool>,
    pub blocks_visibility: Option<bool>,
    pub door_open: Option<bool>,
    pub entry_trigger: Option<EntryTrigger>,
    pub interactable: Option<bool>,
    pub interactable_object: Option<InteractableObject>,
    pub leaf: Option<bool>,
    pub tree: Option<bool>,
    pub energy_reserve: Option<RawEnergyReserve>,
    pub viewshed: Option<RawViewshed>,
    pub cow: Option<bool>,
    pub solo_reproduction: Option<SoloReproduction>,
}

#[derive(Deserialize, Debug)]
pub struct EntryTrigger {
    pub effects: HashMap<String, String>,
}
