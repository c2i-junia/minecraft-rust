use crate::camera::BlockRaycastSet;
use shared::CHUNK_SIZE;
use crate::world::{
    Chunk, MaterialResource, QueuedEvents, WorldMap, WorldRenderRequestUpdateEvent
};
use crate::{world, GameState};
use bevy::asset::Assets;
use bevy::math::IVec3;
use bevy::pbr::PbrBundle;
use bevy::prelude::*;
use bevy::prelude::{Commands, Mesh, Res, Transform};
use bevy_mod_raycast::deferred::RaycastMesh;
use shared::world::{global_block_to_chunk_pos, BlockData, Registry, SIX_OFFSETS};


fn update_chunk(
    chunk: &mut Chunk,
    chunk_pos: &IVec3,
    material_resource: &Res<MaterialResource>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    world_map: &mut WorldMap,
    r_blocks: &Registry<BlockData>,
) {
    let texture = material_resource.atlas_texture.clone().unwrap();
    let new_mesh = world::meshing::generate_chunk_mesh(world_map, &chunk, chunk_pos, r_blocks);

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
            .id();

        let ch = world_map.map.get_mut(chunk_pos).unwrap();
        ch.entity = Some(new_entity);
    }
    // println!("Chunk updated : len={}", chunk.map.len());
}

pub fn world_render_system(
    mut world_map: ResMut<WorldMap>,
    material_resource: Res<MaterialResource>,
    mut commands: Commands,
    mut ev_render: EventReader<WorldRenderRequestUpdateEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut queued_events: Local<QueuedEvents>,
    r_blocks: Res<Registry<BlockData>>,
) {
    for event in ev_render.read() {
        queued_events.events.insert(*event);
    }

    if material_resource.atlas_texture.is_none() {
        // let's wait until it's ready
        return;
    }

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
            if let Some(mut chunk) = cloned_map.map.get_mut(pos) {
                update_chunk(
                    &mut chunk,
                    pos,
                    &material_resource,
                    &mut commands,
                    &mut meshes,
                    &mut world_map,
                    &r_blocks,
                );
            }
        }

    }
    queued_events.events.clear();
}