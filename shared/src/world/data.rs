use bevy::math::IVec3;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::RegistryId;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Chunk {
    pub map: HashMap<IVec3, RegistryId>,
    /// Timestamp marking the last update this chunk has received
    pub ts: u64
}

#[derive(Resource, Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct WorldMap {
    pub map: HashMap<IVec3, Chunk>,
    pub chunks_to_update: Vec<IVec3>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Item {
    pub id: RegistryId,
    pub nb: u32,
}
