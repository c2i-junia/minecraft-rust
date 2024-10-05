use crate::init::TickCounter;
use bevy_ecs::prelude::*;
use bevy_ecs::system::ResMut;
use bevy_renet::renet::{DefaultChannel, RenetServer};
use bincode::Options;
use shared::messages::{ServerToClientMessage, WorldUpdate};
use shared::world::WorldMap;

pub fn broadcast_world_state(
    mut server: ResMut<RenetServer>,
    ticker: Res<TickCounter>,
    world_map: Res<WorldMap>,
) {
    if ticker.tick % 60 != 0 {
        return;
    }
    println!("Broadcast world update");
    let payload = bincode::options()
        .serialize(&ServerToClientMessage::WorldUpdate(to_network(
            &world_map,
            ticker.tick,
        )))
        .unwrap();
    server.broadcast_message(DefaultChannel::ReliableUnordered, payload);
}
fn to_network(world_map: &Res<WorldMap>, tick: u64) -> WorldUpdate {
    WorldUpdate {
        tick,
        new_world: WorldMap {
            map: world_map.map.clone(),
        },
    }
}
