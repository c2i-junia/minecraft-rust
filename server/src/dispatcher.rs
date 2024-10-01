use crate::chat;
use crate::chat::{setup_chat_resources, ChatMessageEvent};
use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};
use bincode::Options;
use shared::messages::{ChatConversation, ChatMessage};

#[derive(Resource)]
pub struct BroadcastTimer {
    pub timer: Timer,
}

pub fn setup_resources_and_events(app: &mut App) {
    app.insert_resource(BroadcastTimer {
        timer: Timer::from_seconds(2.0, TimerMode::Repeating),
    });

    setup_chat_resources(app);
}

pub fn register_systems(app: &mut App) {
    app.add_systems(Update, server_update_system);

    app.add_systems(Update, chat::broadcast_chat_messages);
}

fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut chat_conversation: ResMut<ChatConversation>,
    mut ev_chat: EventWriter<ChatMessageEvent>,
) {
    for event in server_events.read() {
        println!("event received");
        match event {
            ServerEvent::ClientConnected { client_id } => {
                println!("Player {} connected.", client_id);
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                println!("Player {} disconnected: {}", client_id, reason);
            }
        }
    }

    for client_id in server.clients_id() {
        while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered)
        {
            let parsed_message = match bincode::options().deserialize::<ChatMessage>(&message) {
                Ok(data) => data,
                Err(e) => {
                    println!(
                        "Failed to parse incoming chat message: {} / {:?}",
                        e, &message
                    );
                    continue;
                }
            };
            println!("Chat message received: {:?}", &parsed_message);
            chat_conversation.messages.push(parsed_message);
            ev_chat.send(ChatMessageEvent);
        }
    }
}
