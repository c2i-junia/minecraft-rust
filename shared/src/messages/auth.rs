use serde::{Deserialize, Serialize};

use super::PlayerSpawnEvent;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AuthRegisterRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AuthRegisterResponse {
    pub username: String,
    pub session_token: u128,
    pub spawn_event: PlayerSpawnEvent,
}
