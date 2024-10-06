use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};
use bevy_app::ScheduleRunnerPlugin;
use bevy_renet::renet::transport::NetcodeServerTransport;
use bevy_renet::renet::RenetServer;
use bevy_renet::RenetServerPlugin;
use std::collections::HashMap;
use std::fmt::Debug;
use std::time::{Duration, SystemTime};

use crate::dispatcher;
use bevy_renet::renet::transport::{ServerAuthentication, ServerConfig};
use bevy_renet::transport::NetcodeServerPlugin;
use std::net::{SocketAddr, UdpSocket};

#[derive(Debug, Default, Resource)]
pub struct ServerLobby {
    pub players: HashMap<u128, String>,
}

#[allow(dead_code)]
pub fn acquire_local_ephemeral_udp_socket() -> UdpSocket {
    acquire_socket_by_port(0)
}

pub fn acquire_socket_by_port(port: u16) -> UdpSocket {
    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse().unwrap();
    UdpSocket::bind(addr).unwrap()
}

pub fn add_netcode_network(app: &mut App, socket: UdpSocket) {
    app.add_plugins(NetcodeServerPlugin);

    let server = RenetServer::new(default());

    let granted_addr = &socket.local_addr().unwrap();

    let current_time: Duration = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    let server_config = ServerConfig {
        current_time,
        max_clients: 64,
        protocol_id: shared::PROTOCOL_ID,
        public_addresses: vec![*granted_addr],
        authentication: ServerAuthentication::Unsecure,
    };

    let transport = NetcodeServerTransport::new(server_config, socket).unwrap();
    app.insert_resource(server);
    app.insert_resource(transport);
}

pub fn init(socket: UdpSocket) {
    println!("Starting server on {}", socket.local_addr().unwrap());
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

    add_netcode_network(&mut app, socket);

    dispatcher::setup_resources_and_events(&mut app);

    dispatcher::register_systems(&mut app);

    setup_heartbeat(&mut app);

    app.run();
}

#[derive(Resource)]
pub struct TickCounter {
    pub(crate) tick: u64,
}

#[derive(Resource)]
struct HeartbeatTimer {
    timer: Timer,
}

fn setup_heartbeat(app: &mut App) {
    app.insert_resource(HeartbeatTimer {
        timer: Timer::new(Duration::from_secs(1), TimerMode::Repeating),
    });
    app.insert_resource(TickCounter { tick: 0 });
    app.add_systems(Update, heartbeat_system);
}

fn heartbeat_system(
    time: Res<Time>,
    mut ticker: ResMut<TickCounter>,
    mut timer: ResMut<HeartbeatTimer>,
) {
    ticker.tick += 1;
    timer.timer.tick(time.delta());
    if timer.timer.finished() {
        println!("Server heartbeat, tick={}", ticker.tick);
    }
}
