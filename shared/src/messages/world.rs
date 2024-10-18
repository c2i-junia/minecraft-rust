use std::collections::HashMap;

use crate::world::Chunk;
use bevy::math::IVec3;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WorldUpdate {
    pub tick: u64,
    pub new_map: HashMap<IVec3, Chunk>,
}
