mod auth;
mod chat;
mod player;
mod system;
mod world;

pub use auth::*;
pub use chat::*;
pub use player::*;
use serde::{Deserialize, Serialize};
pub use system::*;
pub use world::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ClientToServerMessage {
    AuthRegisterRequest(AuthRegisterRequest),
    ChatMessage(ChatMessage),
    ShutdownOrder(ShutdownOrder),
    PlayerInputs(PlayerInputs),
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ServerToClientMessage {
    AuthRegisterResponse(AuthRegisterResponse),
    ChatConversation(ChatConversation),
    WorldUpdate(WorldUpdate),
}
