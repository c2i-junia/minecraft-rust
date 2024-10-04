use crate::chat;
use crate::chat::{setup_chat_resources, ChatMessageEvent};
use crate::init::ServerLobby;
use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};
use bincode::Options;
use rand::random;
use shared::messages::{AuthRegisterRequest, AuthRegisterResponse, ChatConversation, ChatMessage};

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
    mut lobby: ResMut<ServerLobby>,
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
            match bincode::options().deserialize::<AuthRegisterRequest>(&message) {
                Ok(auth_req) => {
                    println!("Auth request received {:?}", auth_req);

                    if lobby.players.values().any(|v| *v == auth_req.username) {
                        println!("Username already in map: {}", &auth_req.username);
                        return;
                    }

                    let new_session_token = generate_session_token();
                    lobby
                        .players
                        .insert(new_session_token, auth_req.username.clone());
                    println!("New map: {:?}", lobby);
                    // TODO: add cleanup system if no heartbeat
                    let msg = &AuthRegisterResponse {
                        username: auth_req.username,
                        session_token: new_session_token,
                    };
                    let payload = bincode::options().serialize(msg).unwrap();
                    server.send_message(client_id, DefaultChannel::ReliableOrdered, payload);
                }
                Err(_e) => {}
            }

            match bincode::options().deserialize::<ChatMessage>(&message) {
                Ok(parsed_message) => {
                    println!("Chat message received: {:?}", &parsed_message);
                    chat_conversation.messages.push(parsed_message);
                    ev_chat.send(ChatMessageEvent);
                }
                Err(_e) => {}
            };
        }
    }
}

fn generate_session_token() -> u128 {
    let random_value: u128 = random();
    random_value
}
