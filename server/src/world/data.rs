use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use shared::world::RegistryId;
use shared::world::ServerChunk;
use std::collections::HashMap;

pub const SAVE_PATH: &str = "saves/";

#[derive(Serialize, Deserialize)]
pub struct Save {
    pub map: HashMap<IVec3, ServerChunk>,
    // pub player_pos: Vec3,
    // pub inventory: HashMap<RegistryId, Item>,
    pub id_to_block: HashMap<RegistryId, String>,
    // pub id_to_item: HashMap<RegistryId, String>,
}
