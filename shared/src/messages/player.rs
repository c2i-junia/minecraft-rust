use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum NetworkPlayerInput {
    Forward,
    Right,
    Backward,
    Left,
    Jump,
    ToggleFlyMode,
    FlyUp,
    FlyDown,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PlayerInputs {
    pub tick: u64,
    pub actions: Vec<NetworkPlayerInput>,
    pub direction: Vec3,
}
