use crate::dispatcher::BroadcastTimer;
use bevy::prelude::*;
use bevy_renet::renet::RenetServer;
use bincode::Options;
use shared::messages::ChatConversation;
use shared::ServerChannel;

pub fn setup_chat_resources(app: &mut App) {
    app.insert_resource(ChatConversation { ..default() });
}

pub fn broadcast_chat_messages(
    mut server: ResMut<RenetServer>,
    chat_messages: Res<ChatConversation>,
    time: Res<Time>,
    mut timer: ResMut<BroadcastTimer>,
) {
    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        println!(
            "Broadcasting chat history, {} messages",
            chat_messages.messages.len()
        );
        let cm: ChatConversation = chat_messages.into_inner().clone();
        let serialized = bincode::options().serialize(&cm).unwrap();
        println!("data {:?}", cm);
        println!("serialized: {:?}", serialized);
        server.broadcast_message(ServerChannel::ServerMessage, serialized);
    }
}
