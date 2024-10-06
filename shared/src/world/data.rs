use bevy::math::IVec3;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::{BlockId, ItemId};

pub const CHUNK_SIZE: i32 = 16;

// #[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
// pub enum Block {
//     Grass,
//     Dirt,
//     Stone,
//     Bedrock,
// }

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Chunk {
    pub map: HashMap<IVec3, BlockId>,
}

#[derive(Resource, Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct WorldMap {
    pub map: HashMap<IVec3, Chunk>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct Item {
    pub id: ItemId,
    pub nb: u32,
}
