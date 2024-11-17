use bevy::{math::IVec3, prelude::ResMut};
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::Options;
use shared::messages::{ChatMessage, ClientToServerMessage, SaveWorldRequest};

pub enum NetworkAction {
    ChatMessage(String),
    WorldUpdateRequest {
        requested_chunks: Vec<IVec3>,
        player_chunk_pos: IVec3,
        render_distance: u32,
    },
    SaveWorldRequest,
}

pub fn send_network_action(client: &mut ResMut<RenetClient>, action: NetworkAction) {
    match action {
        NetworkAction::ChatMessage(msg) => {
            let timestamp_ms = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            let input_message = bincode::options()
                .serialize(&ClientToServerMessage::ChatMessage(ChatMessage {
                    author_name: "User".into(),
                    content: msg,
                    date: timestamp_ms,
                }))
                .unwrap();

            client.send_message(DefaultChannel::ReliableOrdered, input_message);
        }
        NetworkAction::WorldUpdateRequest {
            requested_chunks,
            player_chunk_pos,
            render_distance,
        } => {
            let input_message = bincode::options()
                .serialize(&ClientToServerMessage::WorldUpdateRequest {
                    player_chunk_position: player_chunk_pos,
                    requested_chunks,
                    render_distance,
                })
                .unwrap();

            client.send_message(DefaultChannel::ReliableOrdered, input_message);
        }
        NetworkAction::SaveWorldRequest => {
            let save_request =
                ClientToServerMessage::SaveWorldRequest(SaveWorldRequest { session_token: 0 });

            let input_message = bincode::options()
                .serialize(&save_request)
                .expect("Failed to serialize SaveWorldRequest");

            client.send_message(DefaultChannel::ReliableOrdered, input_message);
        }
    }
}
