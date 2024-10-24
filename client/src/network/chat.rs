use crate::{network::api::{send_network_action, NetworkAction}, world::RenderDistance};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use shared::messages::ChatConversation;

#[derive(Resource, Default, Debug)]
pub struct CachedChatConversation {
    pub last_update: u64,
    pub data: Option<ChatConversation>,
}

pub fn send_chat_message(client: &mut ResMut<RenetClient>, render_distance: &RenderDistance, msg: &str) {
    send_network_action(client, render_distance, NetworkAction::ChatMessage(msg.into()));
}

pub fn update_cached_chat_state(
    chat_state: &mut ResMut<CachedChatConversation>,
    new_state: ChatConversation,
) {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    chat_state.last_update = now;
    chat_state.data = Some(new_state);

    println!("new CachedChatConversation: {:?}", &chat_state);
}
