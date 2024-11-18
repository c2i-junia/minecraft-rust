use crate::network::api::{send_network_action, NetworkAction};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;

// Send save request to server
pub fn send_save_request_to_server(client: &mut ResMut<RenetClient>) {
    send_network_action(client, NetworkAction::SaveWorldRequest);
    debug!("Save request sent to server.");
}
