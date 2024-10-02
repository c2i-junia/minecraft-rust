use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AuthRegisterRequest {
    username: String,
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct AuthWhoAmIRequest {
    username: String,
}
