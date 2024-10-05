use bevy::math::IVec3;
use bevy::prelude::{Component, Resource};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub const CHUNK_SIZE: i32 = 16;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Block {
    Grass,
    Dirt,
    Stone,
    Bedrock,
}

#[derive(Component, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct BlockWrapper {
    pub kind: Block,
}

impl BlockWrapper {
    pub fn new(kind: Block) -> BlockWrapper {
        BlockWrapper { kind }
    }
}

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Chunk {
    pub map: HashMap<IVec3, BlockWrapper>,
}

#[derive(Resource, Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct WorldMap {
    pub map: HashMap<IVec3, Chunk>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub nb: u32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum ItemId {
    Grass,
    Dirt,
    Stone,
    Bedrock,
}