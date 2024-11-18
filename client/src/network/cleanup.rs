use crate::network::TargetServer;
use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::Options;
use shared::messages::{ClientToServerMessage, ShutdownOrder};

pub fn terminate_server_connection(
    mut client: ResMut<RenetClient>,
    mut target: ResMut<TargetServer>,
) {
    info!("Terminating server connection");
    let order = ClientToServerMessage::ShutdownOrder(ShutdownOrder {
        session_token: target.session_token.unwrap(),
    });
    let payload = bincode::options().serialize(&order).unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, payload);

    target.address = None;
    target.username = None;
    target.session_token = None;
}
