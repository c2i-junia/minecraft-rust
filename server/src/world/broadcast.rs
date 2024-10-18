use std::collections::HashMap;

use crate::init::TickCounter;
use crate::world::generation::generate_chunk;
use bevy::math::IVec3;
use bevy_ecs::prelude::*;
use bevy_ecs::system::ResMut;
use bevy_renet::renet::{ClientId, DefaultChannel, RenetServer};
use bincode::Options;
use shared::messages::{ServerToClientMessage, WorldUpdate};
use shared::world::{chunk_in_radius, BlockData, Chunk, Registry, WorldMap};

use super::data::WorldSeed;

#[derive(Event, Debug)]
pub struct WorldUpdateRequestEvent{
    pub client: ClientId,
    pub chunks: Vec<IVec3>,
    pub player_chunk_position: IVec3
}

pub fn send_world_update(
    mut server: ResMut<RenetServer>,
    ticker: Res<TickCounter>,
    seed: Res<WorldSeed>,
    mut world_map: ResMut<WorldMap>,
    r_blocks: Res<Registry<BlockData>>,
    mut ev_update: EventReader<WorldUpdateRequestEvent>,
) {
    for event in ev_update.read() {
        let payload = bincode::options()
            .serialize(&ServerToClientMessage::WorldUpdate(WorldUpdate {
                tick: ticker.tick,
                new_map: {
                    let mut map: HashMap<IVec3, Chunk> = HashMap::new();
                    for c in event.chunks.iter() {
                        if chunk_in_radius(&event.player_chunk_position, c, 3) {
                            let chunk = world_map.map.get(c);

                            // If chunk already exists, transmit it to client
                            if let Some(chunk) = chunk {
                                map.insert(*c, chunk.clone());
                            } else {
                                // If chunk does not exists, generate it before transmitting it
                                let chunk = generate_chunk(*c, seed.0, &r_blocks);

                                // If chunk is empty, do not create it to prevent unnecessary data transmission
                                if chunk.map.len() == 0 {
                                    continue;
                                }

                                map.insert(*c, chunk.clone());
                                world_map.map.insert(*c, chunk);
                            }
                        }
                    }
                    println!("Update event yippeee :D    len={}", map.len());
                    map
                },
            }))
            .unwrap();

        server.send_message(event.client, DefaultChannel::ReliableUnordered, payload);
    }
}

pub fn broadcast_world_state(
    mut server: ResMut<RenetServer>,
    ticker: Res<TickCounter>,
    mut world_map: ResMut<WorldMap>,
) {
    if ticker.tick % 60 != 0 {
        return;
    }
    println!("Broadcast world update");
    let payload = bincode::options()
        .serialize(&ServerToClientMessage::WorldUpdate(to_network(
            &mut world_map,
            ticker.tick,
        )))
        .unwrap();
    server.broadcast_message(DefaultChannel::ReliableUnordered, payload);
}
fn to_network(world_map: &mut WorldMap, tick: u64) -> WorldUpdate {
    WorldUpdate {
        tick,
        new_map: {
            let mut m: HashMap<IVec3, Chunk> = HashMap::new();
            // Only send chunks that must be updated
            for v in world_map.chunks_to_update.iter() {
                m.insert(*v, world_map.map.get(v).unwrap().clone());
            }
            // Chunks are up do date, clear the vector
            world_map.chunks_to_update.clear();
            m
        },
    }
}
