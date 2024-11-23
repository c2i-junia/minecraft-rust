use std::{collections::HashMap, time::Instant};
use std::f32::consts::PI;

use crate::world::{ClientChunk, ClientWorldMap};
use bevy::{
    math::IVec3,
    prelude::*,
    render::mesh::{Indices, PrimitiveTopology},
};
use shared::world::{to_global_pos, BlockDirection};

use super::voxel::{Face, FaceDirection, VoxelShape};

#[derive(Copy, Clone)]
pub struct UvCoords {
    u0: f32,
    u1: f32,
    v0: f32,
    v1: f32,
}

impl UvCoords {
    pub fn new(u0: f32, u1: f32, v0: f32, v1: f32) -> Self {
        Self { u0, u1, v0, v1 }
    }
}

pub(crate) fn generate_chunk_mesh(
    world_map: &ClientWorldMap,
    chunk: &ClientChunk,
    chunk_pos: &IVec3,
    block_uvs: &HashMap<String, UvCoords>,
) -> Mesh {
    let start = Instant::now();

    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut colors = Vec::new();

    let mut indices_offset = 0;

    let should_render_front_face = |global_block_pos: &IVec3| -> bool {
        let front_offset = IVec3::new(0, 0, -1);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + front_offset))
            .is_none()
    };

    let should_render_back_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(0, 0, 1);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    let should_render_left_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(-1, 0, 0);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    let should_render_right_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(1, 0, 0);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    let should_render_bottom_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(0, -1, 0);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    let should_render_top_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(0, 1, 0);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    for (local_block_pos, block) in chunk.map.iter() {
        let x = local_block_pos.x as f32;
        let y = local_block_pos.y as f32;
        let z = local_block_pos.z as f32;

        let global_block_pos = &to_global_pos(chunk_pos, local_block_pos);

        if is_block_surrounded(world_map, chunk_pos, local_block_pos) {
            continue;
        }

        let mut local_vertices: Vec<[f32; 3]> = vec![];
        let mut local_indices: Vec<u32> = vec![];
        let mut local_normals: Vec<[f32; 3]> = vec![];
        let mut local_uvs: Vec<[f32; 2]> = vec![];
        let mut local_colors: Vec<[f32; 4]> = vec![];

        let voxel = VoxelShape::create_from_block(block);

        for face in voxel.faces.iter() {
            let uv_coords: &UvCoords;

            if let Some(uvs) = block_uvs.get(&face.texture) {
                uv_coords = uvs;
            } else {
                uv_coords = block_uvs.get("_Default").unwrap();
            }

            let should_render = match face.direction {
                FaceDirection::Back => should_render_back_face(global_block_pos),
                FaceDirection::Front => should_render_front_face(global_block_pos),
                FaceDirection::Bottom => should_render_bottom_face(global_block_pos),
                FaceDirection::Top => should_render_top_face(global_block_pos),
                FaceDirection::Left => should_render_left_face(global_block_pos),
                FaceDirection::Right => should_render_right_face(global_block_pos),
                // FaceDirection::Inset => true,
            };

            if should_render {
                render_face(
                    &mut local_vertices,
                    &mut local_indices,
                    &mut local_normals,
                    &mut local_uvs,
                    &mut local_colors,
                    &mut indices_offset,
                    face,
                    uv_coords,
                );
            }
        }

        let local_vertices: Vec<[f32; 3]> = local_vertices
            .iter()
            .map(|v| {
                let v = rotate_vertices(v, &block.direction);
                [
                    v[0] + x,
                    if block.flipped { 1. - v[1] } else { v[1] } + y,
                    v[2] + z,
                ]
            })
            .collect();

        vertices.extend(local_vertices);
        indices.extend(local_indices);
        normals.extend(local_normals);
        uvs.extend(local_uvs);
        colors.extend(local_colors);
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    debug!("Render time : {:?}", Instant::now() - start);

    mesh
}

pub(crate) fn is_block_surrounded(
    world_map: &ClientWorldMap,
    chunk_pos: &IVec3,
    local_block_pos: &IVec3,
) -> bool {
    let global_block_pos = to_global_pos(chunk_pos, local_block_pos);

    for offset in &shared::world::SIX_OFFSETS {
        let neighbor_pos = global_block_pos + *offset;

        // Check if the block exists at the neighboring position
        if world_map.get_block_by_coordinates(&neighbor_pos).is_none() {
            return false;
        }
    }

    true
}

pub fn rotate_vertices(v: &[f32; 3], direction: &BlockDirection) -> [f32; 3] {
    let angle = match *direction {
        BlockDirection::Front => 0.,
        BlockDirection::Right => -PI / 2.,
        BlockDirection::Left => PI / 2.,
        BlockDirection::Back => PI,
    };

    [
        angle.cos() * v[0] + angle.sin() * v[2],
        v[1],
        (-angle).sin() * v[0] + angle.cos() * v[2],
    ]
}

#[allow(clippy::too_many_arguments)]
fn render_face(
    local_vertices: &mut Vec<[f32; 3]>,
    local_indices: &mut Vec<u32>,
    local_normals: &mut Vec<[f32; 3]>,
    local_uvs: &mut Vec<[f32; 2]>,
    local_colors: &mut Vec<[f32; 4]>,
    indices_offset: &mut u32,
    face: &Face,
    uv_coords: &UvCoords,
) {
    local_vertices.extend(face.vertices.iter());

    local_indices.extend(face.indices.iter().map(|x| x + *indices_offset));
    *indices_offset += face.vertices.len() as u32;

    local_normals.extend(face.normals.iter());

    local_colors.extend(face.colors.iter());

    local_uvs.extend(face.uvs.iter().map(|uv| {
        [
            (uv[0] + uv_coords.u0).min(uv_coords.u1),
            (uv[1] + uv_coords.v0).min(uv_coords.v1),
        ]
    }));
}
