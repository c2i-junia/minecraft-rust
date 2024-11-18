use bevy::prelude::*;
use bevy_renet::{renet::RenetClient, RenetClientPlugin};
use rand::Rng;
use shared::GameServerConfig;

use crate::menu::solo::SelectedWorld;
use crate::network::world::update_world_from_network;
use crate::network::{update_cached_chat_state, CachedChatConversation};
use crate::player::Player;
use crate::world::{RenderDistance, WorldRenderRequestUpdateEvent};
use crate::GameState;
use bevy_renet::renet::transport::{
    ClientAuthentication, NetcodeClientTransport, NetcodeTransportError,
};
use bevy_renet::renet::DefaultChannel;
use bevy_renet::transport::NetcodeClientPlugin;
use bincode::Options;
use shared::messages::{AuthRegisterRequest, ChatConversation, ClientToServerMessage};
use std::net::SocketAddr;
use std::{net::UdpSocket, thread, time::SystemTime};

use crate::world::ClientWorldMap;

#[derive(Resource, Debug, Clone)]
pub struct TargetServer {
    pub address: Option<SocketAddr>,
    pub username: Option<String>,
    pub session_token: Option<u128>,
}

pub fn add_base_netcode(app: &mut App) {
    app.add_plugins(RenetClientPlugin);

    let client = RenetClient::new(default());
    app.insert_resource(client);

    // Setup the transport layer
    app.add_plugins(NetcodeClientPlugin);

    // TODO: change username
    app.insert_resource(TargetServer {
        address: None,
        username: None,
        session_token: None,
    });
}

pub fn launch_local_server_system(
    mut target: ResMut<TargetServer>,
    selected_world: Res<SelectedWorld>,
) {
    if target.address.is_some() {
        debug!("Skipping launch local server");
        return;
    }

    if let Some(world_name) = &selected_world.name {
        info!("Launching local server with world: {}", world_name);

        let socket = server::acquire_local_ephemeral_udp_socket();
        let addr = socket.local_addr().unwrap();
        debug!("Obtained UDP socket: {}", addr);

        let world_name_clone = world_name.clone();
        thread::spawn(move || {
            server::init(
                socket,
                GameServerConfig {
                    world_name: world_name_clone,
                    is_solo: true,
                },
            );
        });

        target.address = Some(addr);
    } else {
        error!("Error: No world selected. Unable to launch the server.");
    }
}

fn poll_reliable_ordered_messages(
    client: &mut ResMut<RenetClient>,
    chat_state: &mut ResMut<CachedChatConversation>,
) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let message = bincode::options().deserialize::<ChatConversation>(&message);
        match message {
            Ok(data) => {
                update_cached_chat_state(chat_state, data);
            }
            Err(e) => error!("err {}", e),
        };
    }
}

fn poll_reliable_unordered_messages(
    client: &mut ResMut<RenetClient>,
    world: &mut ResMut<ClientWorldMap>,
    ev_render: &mut EventWriter<WorldRenderRequestUpdateEvent>,
    player_pos: Query<&Transform, With<Player>>,
    render_distance: Res<RenderDistance>,
) {
    update_world_from_network(client, world, ev_render, player_pos, render_distance);
}

pub fn poll_network_messages(
    mut client: ResMut<RenetClient>,
    mut chat_state: ResMut<CachedChatConversation>,
    mut world: ResMut<ClientWorldMap>,
    mut ev_render: EventWriter<WorldRenderRequestUpdateEvent>,
    player_pos: Query<&Transform, With<Player>>,
    render_distance: Res<RenderDistance>,
) {
    poll_reliable_ordered_messages(&mut client, &mut chat_state);
    poll_reliable_unordered_messages(
        &mut client,
        &mut world,
        &mut ev_render,
        player_pos,
        render_distance,
    );
}

pub fn init_server_connection(mut commands: Commands, target: Res<TargetServer>) {
    let mut rng = rand::thread_rng();
    let random_client_id: u64 = rng.gen();

    let addr = target.address.unwrap();
    commands.add(move |world: &mut World| {
        world.remove_resource::<RenetClient>();
        world.remove_resource::<NetcodeClientTransport>();
        world.remove_resource::<CachedChatConversation>();

        let client = RenetClient::new(default());
        world.insert_resource(client);

        let authentication = ClientAuthentication::Unsecure {
            server_addr: addr,
            client_id: random_client_id,
            user_data: None,
            protocol_id: shared::PROTOCOL_ID,
        };
        let socket = UdpSocket::bind("127.0.0.1:0".parse::<SocketAddr>().unwrap()).unwrap();
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

        world.insert_resource(transport);

        world.insert_resource(CachedChatConversation { ..default() });

        info!("Network subsystem initialized");
    })
}

pub fn network_failure_handler(mut renet_error: EventReader<NetcodeTransportError>) {
    for e in renet_error.read() {
        error!("network error: {}", e);
    }
}

pub fn establish_authenticated_connection_to_server(
    mut client: ResMut<RenetClient>,
    mut target: ResMut<TargetServer>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if target.session_token.is_some() {
        info!(
            "Successfully acquired a session token as {}",
            &target.username.clone().unwrap()
        );
        game_state.set(GameState::Game);
        return;
    }
    debug!("trying to connect... {:?}", target);

    let username = target.username.as_ref().unwrap();

    let auth_msg = ClientToServerMessage::AuthRegisterRequest(AuthRegisterRequest {
        username: username.clone(),
    });
    let auth_msg_encoded = bincode::options().serialize(&auth_msg).unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, auth_msg_encoded);

    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
        let message =
            bincode::options().deserialize::<shared::messages::AuthRegisterResponse>(&message);
        if let Ok(message) = message {
            target.username = Some(message.username);
            target.session_token = Some(message.session_token);
            info!("Connected! {:?}", target);
        }
    }
}
