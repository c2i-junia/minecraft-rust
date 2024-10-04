mod auth;
mod chat_messages;
mod system;

pub use auth::*;
pub use chat_messages::*;
use serde::{Deserialize, Serialize};
pub use system::*;

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
}
