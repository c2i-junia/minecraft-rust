use bevy::prelude::*;
use bevy_renet::{renet::RenetClient, RenetClientPlugin};

use crate::network::ChatConversationBuffer;
use bevy_renet::renet::transport::{
    ClientAuthentication, NetcodeClientTransport, NetcodeTransportError,
};
use bevy_renet::transport::NetcodeClientPlugin;
use bincode::Options;
use shared::messages::{ChatConversation, ChatMessage};
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

    app.add_systems(Update, network_send_message_loop_test);

    app.add_systems(Update, poll_network_messages);

    app.insert_resource(ChatConversationBuffer { ..default() });

    println!("Network subsystem initialized");
}

pub fn network_send_message_loop_test(mut client: ResMut<RenetClient>) {
    let timestamp_ms = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();
    if timestamp_ms % 10 != 0 {
        return;
    }

    let cm = ChatMessage {
        author_name: "Test".into(),
        date: 0,
        content: format!("Hello from client at {}", timestamp_ms),
    };

    let cm_serialized = bincode::options().serialize(&cm).unwrap();

    client.send_message(shared::ClientChannel::ChatMessage, cm_serialized);

    while let Some(message) = client.receive_message(shared::ServerChannel::ServerMessage) {
        println!("Received reply from server: {:?}", message);
    }
}

pub fn poll_network_messages(mut client: ResMut<RenetClient>) {
    while let Some(message) = client.receive_message(shared::ServerChannel::ServerMessage) {
        let message = bincode::options().deserialize::<ChatConversation>(&message);
        match message {
            Ok(data) => println!("ok: {:?}", data),
            Err(e) => println!("err {}", e),
        };
    }
}
