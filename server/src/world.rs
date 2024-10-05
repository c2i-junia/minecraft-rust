use crate::init::TickCounter;
use bevy::utils::default;
use bevy_ecs::prelude::*;
use bevy_ecs::system::ResMut;
use bevy_renet::renet::{DefaultChannel, RenetServer};
use bincode::Options;
use shared::messages::{ServerToClientMessage, WorldUpdate};

pub fn broadcast_world_state(mut server: ResMut<RenetServer>, ticker: Res<TickCounter>) {
    if ticker.tick % 60 != 0 {
        return;
    }
    println!("Broadcast world update");
    let payload = bincode::options()
        .serialize(&ServerToClientMessage::WorldUpdate(WorldUpdate {
            tick: ticker.tick,
            new_world: default(),
        }))
        .unwrap();
    server.broadcast_message(DefaultChannel::Unreliable, payload);
}
