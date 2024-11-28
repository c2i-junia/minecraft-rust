use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::Options;
use shared::messages::ClientToServerMessage;

use crate::player::CurrentPlayerMarker;

pub fn send_player_position_to_server(
    mut client: ResMut<RenetClient>,
    player: Query<&Transform, With<CurrentPlayerMarker>>,
) {
    let msg = ClientToServerMessage::SetPlayerPosition {
        position: player.single().translation,
    };
    let payload = bincode::options().serialize(&msg).unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, payload);
}
