use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ChatMessage {
    pub author_name: String,
    pub date: u64, // timestamp ms
    pub content: String,
}

#[derive(Resource, Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct ChatConversation {
    pub messages: Vec<ChatMessage>,
}
