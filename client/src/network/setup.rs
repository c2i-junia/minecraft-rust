use bevy::log::info;
use naia_bevy_client::{transport::webrtc, Client};
use shared::messages::Auth;

use crate::network::MainClient;

pub fn init_network_socket(mut client: Client<MainClient>) {
    info!("Naia Bevy Client Demo started");

    let socket = webrtc::Socket::new("http://127.0.0.1:14191", client.socket_config());

    client.auth(Auth::new("charlie", "12345"));
    client.connect(socket);
}
