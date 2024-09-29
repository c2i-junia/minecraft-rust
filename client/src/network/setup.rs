use bevy::prelude::*;
use bevy_renet::{renet::RenetClient, RenetClientPlugin};

use bevy_renet::renet::transport::{
    ClientAuthentication, NetcodeClientTransport, NetcodeTransportError,
};
use bevy_renet::transport::NetcodeClientPlugin;
use std::time::UNIX_EPOCH;
use std::{net::UdpSocket, time::SystemTime};

pub fn add_netcode_network(app: &mut App) {
    app.add_plugins(RenetClientPlugin);

    let client = RenetClient::new(shared::connection_config());
    app.insert_resource(client);

    // Setup the transport layer
    app.add_plugins(NetcodeClientPlugin);

    let authentication = ClientAuthentication::Unsecure {
        server_addr: "127.0.0.1:5000".parse().unwrap(),
        client_id: 0,
        user_data: None,
        protocol_id: shared::PROTOCOL_ID,
    };
    let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

    app.insert_resource(transport);

    fn network_failure_handler(mut renet_error: EventReader<NetcodeTransportError>) {
        for e in renet_error.read() {
            println!("network error: {}", e);
        }
    }

    app.add_systems(Update, network_failure_handler);

    app.add_systems(Update, network_send_message);

    println!("Network subsystem initialized");
}

pub fn network_send_message(mut client: ResMut<RenetClient>) {
    let timestamp_ms = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    if timestamp_ms % 50 != 0 {
        return;
    }

    let player_input = "hello from client";
    println!("trying to ping the server with message: {}", player_input);
    let input_message = bincode::serialize(player_input).unwrap();

    client.send_message(shared::ClientChannel::ChatMessage, input_message);

    while let Some(message) = client.receive_message(shared::ServerChannel::ServerMessage) {
        println!("Received reply from server: {:?}", message);
    }
}
