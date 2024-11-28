use std::collections::HashMap;

use crate::world::ServerChunk;
use bevy::math::{IVec3, Vec3};
use serde::{Deserialize, Serialize};

use super::PlayerId;

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
pub struct WorldUpdate {
    pub tick: u64,
    pub new_map: HashMap<IVec3, ServerChunk>,
    pub player_positions: HashMap<PlayerId, Vec3>,
}
