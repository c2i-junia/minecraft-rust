use std::thread::sleep;
use std::time::Duration;

use crate::camera::BlockRaycastSet;
use crate::world::{
    Chunk, MaterialResource, QueuedEvents, WorldMap, WorldRenderRequestUpdateEvent,
};
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

#[derive(Debug, Component)]
pub struct AwaitingMesh(Task<CommandQueue>);

fn update_chunk(
    chunk: &mut Chunk,
    chunk_pos: &IVec3,
    material_resource: &Res<MaterialResource>,
    commands: &mut Commands,
    world_map: &mut WorldMap,
    r_blocks: &Registry<BlockData>,
    task_pool: &AsyncComputeTaskPool,
) {
    let texture = material_resource.atlas_texture.clone().unwrap();

    if chunk.entity.is_some() {
        commands
            .get_entity(chunk.entity.unwrap())
            .unwrap()
            .despawn_descendants()
            .clear()
            .insert(StateScoped(GameState::Game));
    } else {
        chunk.entity = Some(commands.spawn(StateScoped(GameState::Game)).id());
    }

    let chunk_t = Transform::from_xyz(
        (chunk_pos.x * CHUNK_SIZE) as f32,
        (chunk_pos.y * CHUNK_SIZE) as f32,
        (chunk_pos.z * CHUNK_SIZE) as f32,
    );

    let chunk_entity = chunk.entity.unwrap();

    let nmap = world_map.clone();
    let npos = chunk_pos.clone();
    let nblocks = r_blocks.clone();

    let t = task_pool.spawn(async move {
        println!("New thread spawned !");

        let new_mesh = world::meshing::generate_chunk_mesh(&nmap, &npos, &nblocks);

        // Test delay, to see if lag comes from threads being computed or not
        // sleep(Duration::from_millis(200));

        let mut queue = CommandQueue::default();

        // When the mesh is done, add it to the world
        queue.push(move |world: &mut World| {
            let mut system_state: SystemState<ResMut<Assets<Mesh>>> = SystemState::new(world);
            let mesh = system_state.get_mut(world).add(new_mesh);

            if let Some(mut chunk) = world.get_entity_mut(chunk_entity) {
                chunk
                    .insert((
                        StateScoped(GameState::Game),
                        PbrBundle {
                            mesh,
                            material: texture.clone(),
                            transform: chunk_t,
                            ..Default::default()
                        },
                        RaycastMesh::<BlockRaycastSet>::default(),
                    ))
                    .remove::<AwaitingMesh>();

                println!("Processed finished");
            }
        });

        queue
    });

    commands.entity(chunk_entity).insert(AwaitingMesh(t));

    let ch = world_map.map.get_mut(chunk_pos).unwrap();
    ch.entity = Some(chunk_entity);
    // println!("Chunk updated : len={}", chunk.map.len());
}

pub fn world_render_system(
    mut world_map: ResMut<WorldMap>,
    material_resource: Res<MaterialResource>,
    mut commands: Commands,
    mut ev_render: EventReader<WorldRenderRequestUpdateEvent>,
    mut queued_events: Local<QueuedEvents>,
    r_blocks: Res<Registry<BlockData>>,
    mut mesh_tasks: Query<&mut AwaitingMesh>,
) {
    for event in ev_render.read() {
        queued_events.events.insert(*event);
    }

    if material_resource.atlas_texture.is_none() {
        // let's wait until it's ready
        return;
    }

    let pool = AsyncComputeTaskPool::get();

    for event in &queued_events.events {
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

        let mut cloned_map = world_map.clone();

        for pos in chunks_pos_to_reload.iter() {
            if let Some(chunk) = cloned_map.map.get_mut(pos) {
                update_chunk(
                    chunk,
                    pos,
                    &material_resource,
                    &mut commands,
                    &mut world_map,
                    &r_blocks,
                    &pool,
                );
            }
        }
    }

    // Poll rendered chunk meshes
    for mut task in mesh_tasks.iter_mut() {
        if let Some(mut commands_queue) = block_on(future::poll_once(&mut task.0)) {
            // Append the returned command queue to have it execute later
            commands.append(&mut commands_queue);
        }
    }

    queued_events.events.clear();
}
