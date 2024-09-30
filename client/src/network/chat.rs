use crate::network::api::{send_network_action, NetworkAction};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use shared::messages::{ChatConversation, ChatMessage};

#[derive(Resource, Default)]
pub struct ChatConversationBuffer {
    pub last_update: u64,
    pub buffer: Option<ChatConversation>,
}

pub fn send_chat_message(client: &mut ResMut<RenetClient>, msg: &str) {
    send_network_action(client, NetworkAction::ChatMessage(msg.into()));
}

pub fn get_chat_messages_from_buffer(buffer: &Res<ChatConversationBuffer>) -> Vec<ChatMessage> {
    vec![]
}
