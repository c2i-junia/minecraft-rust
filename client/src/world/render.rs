use std::collections::HashMap;
use std::time::Instant;

use crate::camera::BlockRaycastSet;
use crate::world::{MaterialResource, QueuedEvents, WorldMap, WorldRenderRequestUpdateEvent};
use crate::{world, GameState};
use bevy::asset::Assets;
use bevy::ecs::system::SystemState;
use bevy::ecs::world::CommandQueue;
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
    pub meshes: Vec<Task<CommandQueue>>,
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
        commands
            .get_entity(chunk.entity.unwrap())
            .unwrap()
            .despawn_recursive();
        chunk.entity = None;
    }

    if chunk.entity.is_none() {
        let chunk_t = Transform::from_xyz(
            (chunk_pos.x * CHUNK_SIZE) as f32,
            (chunk_pos.y * CHUNK_SIZE) as f32,
            (chunk_pos.z * CHUNK_SIZE) as f32,
        );
        // Cube
        let new_entity = commands
            .spawn((
                StateScoped(GameState::Game),
                PbrBundle {
                    mesh: meshes.add(new_mesh),
                    material: texture.clone(),
                    transform: chunk_t.clone(),
                    ..Default::default()
                },
                RaycastMesh::<BlockRaycastSet>::default(),
            ))
            .with_children(|builder| {
                builder.spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(Cuboid::new(
                        2.,
                        2.,
                        2.,
                    ))),
                    ..Default::default()
                });
            })
            .id();

        println!("Chunk spawned with entity {:?} at pos {:?}, new map length : {}", new_entity, chunk_t, world_map.map.len());

        let ch = world_map.map.get_mut(chunk_pos).unwrap();
        ch.entity = Some(new_entity);
    }
    // println!("Chunk updated : len={}", chunk.map.len());
}

pub fn world_render_system(
    world_map: Res<WorldMap>,
    material_resource: Res<MaterialResource>,
    mut ev_render: EventReader<WorldRenderRequestUpdateEvent>,
    mut queued_events: Local<QueuedEvents>,
    mut queued_meshes: Local<QueuedMeshes>,
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

            let mut queue = CommandQueue::default();

            println!(
                "============= Meshing done in {:?} ",
                Instant::now() - start
            );

            // Push a command to the queue, to spawn all chunks
            queue.push(move |world: &mut World| {
                let start = Instant::now();
                // Request all necessary resources from the current world state
                let mut system_state: SystemState<(
                    Res<MaterialResource>,
                    Commands,
                    ResMut<Assets<Mesh>>,
                    ResMut<WorldMap>,
                )> = SystemState::new(world);

                // Then, retrieve these resources
                let (material_resource, mut commands, mut meshes, mut world_map) =
                    system_state.get_mut(world);

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

                println!(">>>>>>>>>>>>>>>>>> Chunks spawned in {:?} ", Instant::now() - start);
            });
            queue
        });

        queued_meshes.meshes.push(t);
    }

    // Iterate through queued meshes to see if they are completed
    queued_meshes.meshes.retain_mut(|task| {
        // If completed, then push the command to the command queue and delete it from the meshing queue
        if let Some(mut commands_queue) = block_on(future::poll_once(task)) {
            // Append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
            false
        } else {
            // Else, keep the task until it is done
            true
        }
    });

    println!("Current number of chunks : {}", world_map.map.len());

    queued_events.events.clear();
}
