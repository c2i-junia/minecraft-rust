use bevy::prelude::Vec3;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum NetworkAction {
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
    pub actions: Vec<NetworkAction>,
    pub direction: Vec3,
}
