use crate::chat::{setup_chat_resources, ChatMessageEvent};
use crate::init::ServerLobby;
use crate::{chat, world};
use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetServer, ServerEvent};
use bincode::Options;
use rand::random;
use shared::messages::{AuthRegisterResponse, ChatConversation, ClientToServerMessage};

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

    app.add_systems(Update, world::broadcast_world_state);
}

fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut chat_conversation: ResMut<ChatConversation>,
    mut ev_chat: EventWriter<ChatMessageEvent>,
    mut lobby: ResMut<ServerLobby>,
    mut ev_app_exit: EventWriter<AppExit>,
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
            println!("msg received {:?}", &message);

            let msg = bincode::options().deserialize::<ClientToServerMessage>(&message);
            let msg = match msg {
                Ok(msg) => msg,
                Err(e) => {
                    println!("Failed to parse incoming message: {}", e);
                    continue;
                }
            };

            match msg {
                ClientToServerMessage::AuthRegisterRequest(auth_req) => {
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
                ClientToServerMessage::ChatMessage(chat_msg) => {
                    println!("Chat message received: {:?}", &chat_msg);
                    chat_conversation.messages.push(chat_msg);
                    ev_chat.send(ChatMessageEvent);
                }
                ClientToServerMessage::ShutdownOrder(order) => {
                    println!("Received shutdown order... {:?}", order);
                    // TODO: add permission checks
                    println!("Server is going down...");
                    ev_app_exit.send(AppExit::Success);
                }
            }
        }
    }
}

fn generate_session_token() -> u128 {
    let random_value: u128 = random();
    random_value
}
