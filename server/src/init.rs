use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_app::ScheduleRunnerPlugin;
use bevy_renet::renet::transport::NetcodeServerTransport;
use bevy_renet::renet::RenetServer;
use bevy_renet::RenetServerPlugin;
use std::fmt::Debug;
use std::time::{Duration, SystemTime};

use crate::dispatcher;
use bevy_renet::renet::transport::{ServerAuthentication, ServerConfig};
use bevy_renet::transport::NetcodeServerPlugin;
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    //pub players: HashMap<ClientId, Entity>,
}

pub fn add_netcode_network(app: &mut App, public_addr: SocketAddr) {
    app.add_plugins(NetcodeServerPlugin);

    let server = RenetServer::new(default());

    let socket = UdpSocket::bind(public_addr).unwrap();
    let current_time: std::time::Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: shared::PROTOCOL_ID,
        public_addresses: vec![public_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    println!(
        "UDP socket granted by the OS: {}",
        &socket.local_addr().unwrap()
    );

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(server);
    app.insert_resource(transport);
}

pub fn init(addr: SocketAddr) {
    println!("Requesting a sever start on {}", addr);
    let mut app = App::new();
    app.add_plugins(
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
    );

    app.add_plugins(RenetServerPlugin);
    app.add_plugins(FrameTimeDiagnosticsPlugin);
    app.add_plugins(LogDiagnosticsPlugin::default());

    app.insert_resource(ServerLobby::default());

    add_netcode_network(&mut app, addr);

    dispatcher::setup_resources_and_events(&mut app);

    dispatcher::register_systems(&mut app);

    app.run();
}
