use bevy::prelude::ResMut;
use bevy_renet::renet::RenetClient;
use std::time::UNIX_EPOCH;

pub enum NetworkAction {
    ChatMessage(String),
}

pub fn send_network_message(mut client: ResMut<RenetClient>, action: NetworkAction) {
    let timestamp_ms = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    if timestamp_ms % 50 != 0 {
        return;
    }

    match action {
        NetworkAction::ChatMessage(msg) => {
            let input_message = bincode::serialize(&msg).unwrap();

            client.send_message(shared::ClientChannel::ChatMessage, input_message);
        }
    }
}
