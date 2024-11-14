use std::collections::HashMap;
use std::time::Instant;

use crate::camera::BlockRaycastSet;
use crate::world::{MaterialResource, QueuedEvents, WorldMap, WorldRenderRequestUpdateEvent};
use crate::{world, GameState};
use bevy::asset::Assets;
use bevy::math::IVec3;
use bevy::pbr::PbrBundle;
use bevy::prelude::*;
use bevy::prelude::{Commands, Mesh, Res, Transform};
use bevy::tasks::{block_on, futures_lite::future, AsyncComputeTaskPool, Task};
use bevy_mod_raycast::deferred::RaycastMesh;
use shared::world::{global_block_to_chunk_pos, BlockData, Registry, SIX_OFFSETS};
use shared::CHUNK_SIZE;

#[derive(Debug, Default, Resource)]
pub struct QueuedMeshes {
    pub meshes: Vec<Task<HashMap<IVec3, Mesh>>>,
}

fn update_chunk(
    chunk_pos: &IVec3,
    material_resource: &MaterialResource,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    world_map: &mut WorldMap,
    new_mesh: Mesh,
) {
    let chunk = world_map.map.get_mut(chunk_pos).unwrap();
    let texture = material_resource.atlas_texture.clone().unwrap();

    if chunk.entity.is_some() {
        commands.entity(chunk.entity.unwrap()).despawn_recursive();
        chunk.entity = None;
    }

    if chunk.entity.is_none() {
        let chunk_t = Transform::from_xyz(
            (chunk_pos.x * CHUNK_SIZE) as f32,
            (chunk_pos.y * CHUNK_SIZE) as f32,
            (chunk_pos.z * CHUNK_SIZE) as f32,
        );

        let new_entity = commands
            .spawn((
                StateScoped(GameState::Game),
                PbrBundle {
                    mesh: meshes.add(new_mesh),
                    material: texture.clone(),
                    transform: chunk_t,
                    ..Default::default()
                },
                RaycastMesh::<BlockRaycastSet>::default(),
            ))
            // .with_children(|builder| {
            //     builder.spawn(PbrBundle {
            //         mesh: meshes.add(Mesh::from(Cuboid::new(2., 2., 2.))),
            //         ..Default::default()
            //     });
            // })
            .id();

        let ch = world_map.map.get_mut(chunk_pos).unwrap();
        ch.entity = Some(new_entity);
    }
    // println!("Chunk updated : len={}", chunk.map.len());
}

#[allow(clippy::too_many_arguments)]
pub fn world_render_system(
    mut world_map: ResMut<WorldMap>,
    material_resource: Res<MaterialResource>,
    mut ev_render: EventReader<WorldRenderRequestUpdateEvent>,
    mut queued_events: Local<QueuedEvents>,
    mut queued_meshes: Local<QueuedMeshes>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut commands: Commands,
    r_blocks: Res<Registry<BlockData>>,
) {
    for event in ev_render.read() {
        queued_events.events.insert(*event);
    }

    if material_resource.atlas_texture.is_none() {
        // let's wait until it's ready
        return;
    }

    let pool = AsyncComputeTaskPool::get();

    let events = queued_events.events.clone();
    let cloned_map = world_map.clone();
    let cloned_blocks = r_blocks.clone();

    if !events.is_empty() {
        let t = pool.spawn(async move {
            let start = Instant::now();

            let mut chunk_meshes: HashMap<IVec3, Mesh> = HashMap::new();
            for event in &events {
                //println!("world_render_system event {:?}", event);
                let target_chunk_pos = match event {
                    WorldRenderRequestUpdateEvent::ChunkToReload(pos) => pos,
                    WorldRenderRequestUpdateEvent::BlockToReload(pos) => {
                        // Temporary shortcut
                        &global_block_to_chunk_pos(pos)
                    }
                };

                let mut chunks_pos_to_reload = vec![*target_chunk_pos];
                for offset in &SIX_OFFSETS {
                    chunks_pos_to_reload.push(*target_chunk_pos + *offset);
                }

                for pos in chunks_pos_to_reload.iter() {
                    // If chunk has already been rendered, ignore it (concurrent updates may apply to the same chunk)
                    if chunk_meshes.contains_key(pos) {
                        continue;
                    }

                    if let Some(chunk) = cloned_map.map.get(pos) {
                        // If chunk is empty, ignore it
                        if chunk.map.is_empty() {
                            continue;
                        }

                        chunk_meshes.insert(
                            *pos,
                            world::meshing::generate_chunk_mesh(
                                &cloned_map,
                                chunk,
                                pos,
                                &cloned_blocks,
                            ),
                        );
                    }
                }
            }

            println!(
                "============= Meshing done in {:?} ",
                Instant::now() - start
            );
            chunk_meshes
        });

        queued_meshes.meshes.push(t);
    }

    // Iterate through queued meshes to see if they are completed
    queued_meshes.meshes.retain_mut(|task| {
        // If completed, then use the mesh to update the chunk and delete it from the meshing queue
        if let Some(mut chunk_meshes) = block_on(future::poll_once(task)) {
            // Update all the chunks by draining the meshes
            for (chunk_pos, new_mesh) in chunk_meshes.drain() {
                if !world_map.map.contains_key(&chunk_pos) {
                    continue;
                }

                update_chunk(
                    &chunk_pos,
                    &material_resource,
                    &mut commands,
                    &mut meshes,
                    &mut world_map,
                    new_mesh,
                );
            }
            false
        } else {
            // Else, keep the task until it is done
            true
        }
    });

    queued_events.events.clear();
}
