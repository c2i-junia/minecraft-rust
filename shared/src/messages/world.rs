use crate::data::Block;
use bevy::math::IVec3;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct Chunk {
    map: HashMap<IVec3, Block>,
}

#[derive(Resource, Default, Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct WorldMap {
    pub map: HashMap<IVec3, Chunk>,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WorldUpdate {
    pub tick: u64,
    pub new_world: WorldMap,
}
