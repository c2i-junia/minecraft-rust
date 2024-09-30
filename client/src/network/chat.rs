use crate::network::api::{send_network_message, NetworkAction};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use shared::messages::ChatConversation;

#[derive(Resource, Default)]
pub struct ChatConversationBuffer {
    pub last_update: u64,
    pub buffer: Option<ChatConversation>,
}

pub fn send_chat_message(client: ResMut<RenetClient>, msg: String) {
    send_network_message(client, NetworkAction::ChatMessage(msg));
}

pub fn get_chat_messages_from_buffer(
    buffer: &Res<ChatConversationBuffer>,
) -> Option<ChatConversation> {
    None
}
