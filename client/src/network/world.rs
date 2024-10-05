use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::Options;
use shared::messages::ServerToClientMessage;

pub fn update_world_from_network(client: &mut ResMut<RenetClient>) {
    while let Some(bytes) = client.receive_message(DefaultChannel::ReliableUnordered) {
        let msg = bincode::options()
            .deserialize::<ServerToClientMessage>(&bytes)
            .unwrap();
        match msg {
            ServerToClientMessage::WorldUpdate(world_update) => {
                println!(
                    "Received world update, {} chunks received",
                    world_update.new_world.map.len()
                );
            }
            _ => {}
        }
    }
}
