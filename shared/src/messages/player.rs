use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::PlayerId;

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

#[derive(Event, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct PlayerSpawnEvent {
    pub id: PlayerId,
    pub name: String,
    pub position: Vec3,
}
