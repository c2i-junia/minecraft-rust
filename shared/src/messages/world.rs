use crate::world::WorldMap;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct WorldUpdate {
    pub tick: u64,
    pub new_world: WorldMap,
}
