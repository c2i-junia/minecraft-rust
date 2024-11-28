use crate::{
    player::{CurrentPlayerMarker, Player},
    world::ClientChunk,
};
use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::Options;
use shared::{
    messages::{PlayerSpawnEvent, ServerToClientMessage},
    world::{block_to_chunk_coord, chunk_in_radius},
};

use crate::world::ClientWorldMap;

use crate::world::{RenderDistance, WorldRenderRequestUpdateEvent};

use super::api::send_network_action;

pub fn update_world_from_network(
    client: &mut ResMut<RenetClient>,
    world: &mut ResMut<ClientWorldMap>,
    ev_render: &mut EventWriter<WorldRenderRequestUpdateEvent>,
    players: &mut Query<(&mut Transform, &Player), With<Player>>,
    current_player_entity: Query<Entity, With<CurrentPlayerMarker>>,
    render_distance: Res<RenderDistance>,
    ev_spawn: &mut EventWriter<PlayerSpawnEvent>,
) {
    let (player_pos, current_player) = players.get(current_player_entity.single()).unwrap();
    let current_player_id = current_player.id;

    let player_pos = IVec3::new(
        block_to_chunk_coord(player_pos.translation.x as i32),
        0,
        block_to_chunk_coord(player_pos.translation.z as i32),
    );
    let r = render_distance.distance as i32;

    while let Some(bytes) = client.receive_message(DefaultChannel::ReliableUnordered) {
        let msg = bincode::options()
            .deserialize::<ServerToClientMessage>(&bytes)
            .unwrap();

        match msg {
            ServerToClientMessage::WorldUpdate(world_update) => {
                debug!(
                    "Received world update, {} chunks received",
                    world_update.new_map.len()
                );

                trace!("Chunks positions : {:?}", world_update.new_map.keys());

                for (pos, chunk) in world_update.new_map {
                    // If the chunk is not in render distance range or is empty, do not consider it
                    if !chunk_in_radius(&player_pos, &pos, r) || chunk.map.is_empty() {
                        continue;
                    }

                    let chunk = ClientChunk {
                        map: chunk.map,
                        entity: {
                            if let Some(c) = world.map.get(&pos) {
                                c.entity
                            } else {
                                None
                            }
                        },
                    };

                    world.map.insert(pos, chunk);
                    ev_render.send(WorldRenderRequestUpdateEvent::ChunkToReload(pos));
                }

                debug!("Player pos {:?}", world_update.player_positions);

                for (mut transform, player) in players.iter_mut() {
                    debug!("Player found: {} at {:?}", player.name, transform);
                    if player.id == current_player_id {
                        continue;
                    }
                    let vec3 = world_update.player_positions.get(&player.id);
                    if let Some(vec3) = vec3 {
                        let new_transform = Transform::from_translation(*vec3);
                        *transform = new_transform;
                        debug!("Set transform {} => {:?}", player.id, new_transform);
                    }
                }
            }
            ServerToClientMessage::PlayerSpawn(spawn_event) => {
                info!("Received SINGLE spawn event {:?}", spawn_event);
                ev_spawn.send(spawn_event);
            }
            _ => {}
        }
    }
}

pub fn request_world_update(
    client: &mut ResMut<RenetClient>,
    requested_chunks: Vec<IVec3>,
    render_distance: &RenderDistance,
    player_chunk_pos: IVec3,
) {
    send_network_action(
        client,
        super::api::NetworkAction::WorldUpdateRequest {
            requested_chunks,
            player_chunk_pos,
            render_distance: render_distance.distance,
        },
    );
}
