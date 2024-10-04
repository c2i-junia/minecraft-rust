use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AuthRegisterRequest {
    pub username: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AuthRegisterResponse {
    pub username: String,
    pub session_token: u128,
}
