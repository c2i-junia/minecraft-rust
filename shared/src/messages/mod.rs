mod auth;
mod chat;
mod system;
mod world;

pub use auth::*;
pub use chat::*;
use serde::{Deserialize, Serialize};
pub use system::*;
pub use world::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ClientToServerMessage {
    AuthRegisterRequest(AuthRegisterRequest),
    ChatMessage(ChatMessage),
    ShutdownOrder(ShutdownOrder),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ServerToClientMessage {
    AuthRegisterResponse(AuthRegisterResponse),
    ChatConversation(ChatConversation),
    WorldUpdate(WorldUpdate),
}
