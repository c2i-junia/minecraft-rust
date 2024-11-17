use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ShutdownOrder {
    pub session_token: u128,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct SaveWorldRequest {
    pub session_token: u128,
}
