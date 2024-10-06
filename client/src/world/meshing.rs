use crate::constants::CHUNK_SIZE;
use crate::world::WorldMap;
use bevy::math::IVec3;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use shared::world::{to_global_pos, BlockId, ItemBlockRegistry};

#[derive(Copy, Clone)]
struct UvCoords {
    u0: f32,
    u1: f32,
    v0: f32,
    v1: f32,
}

fn get_uv_coords(block: &BlockId, registry: &ItemBlockRegistry) -> UvCoords {
    // should be refactored later
    let res = registry.blocks.get(block).unwrap().uvs;
    UvCoords {
        u0: res[0],
        u1: res[1],
        v0: res[2],
        v1: res[3],
    }
}

pub(crate) fn generate_chunk_mesh(
    world_map: &WorldMap,
    chunk_pos: &IVec3,
    registry: &ItemBlockRegistry,
) -> Mesh {
    let mut vertices: Vec<[f32; 3]> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();

    let mut indices_offset = 0;

    let should_render_front_face = |global_block_pos: &IVec3| -> bool {
        let front_offset = IVec3::new(0, 0, -1);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + front_offset))
            .is_none()
    };

    let render_uvs = |local_uvs: &mut Vec<[f32; 2]>, uv_coords: UvCoords| {
        let UvCoords { u0, u1, v0, v1 } = uv_coords;
        local_uvs.extend(vec![[u0, v0], [u1, v0], [u1, v1], [u0, v1]])
    };

    let render_front_face = |local_vertices: &mut Vec<[f32; 3]>,
                             local_indices: &mut Vec<u32>,
                             local_normals: &mut Vec<[f32; 3]>,
                             local_uvs: &mut Vec<[f32; 2]>,
                             indices_offset: &mut u32,
                             uv_coords: UvCoords| {
        local_vertices.extend(vec![
            [-0.5, -0.5, -0.5], // A 00 Front [0]
            [0.5, -0.5, -0.5],  // B 01 Front [1]
            [0.5, 0.5, -0.5],   // C 02 Front [2]
            [-0.5, 0.5, -0.5],  // D 03 Front [3]
        ]);

        // 0, 3, 2, 2, 1, 0,
        local_indices.extend([0, 3, 2, 2, 1, 0].iter().map(|x| x + *indices_offset));
        *indices_offset += 4;

        local_normals.extend(vec![
            // Front face (-Z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ]);

        render_uvs(local_uvs, uv_coords);
    };

    let should_render_back_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(0, 0, 1);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    let render_back_face = |local_vertices: &mut Vec<[f32; 3]>,
                            local_indices: &mut Vec<u32>,
                            local_normals: &mut Vec<[f32; 3]>,
                            local_uvs: &mut Vec<[f32; 2]>,
                            indices_offset: &mut u32,
                            uv_coords: UvCoords| {
        local_vertices.extend(vec![
            [-0.5, -0.5, 0.5], // E 04 Back [0]
            [0.5, -0.5, 0.5],  // F 05 Back [1]
            [0.5, 0.5, 0.5],   // G 06 Back [2]
            [-0.5, 0.5, 0.5],  // H 07 Back [3]
        ]);

        // 4, 5, 6, 6, 7, 4,
        local_indices.extend([0, 1, 2, 2, 3, 0].iter().map(|x| x + *indices_offset));
        *indices_offset += 4;

        local_normals.extend(vec![
            // Back face (+Z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
        ]);

        render_uvs(local_uvs, uv_coords);
    };

    let should_render_left_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(-1, 0, 0);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    let render_left_face = |local_vertices: &mut Vec<[f32; 3]>,
                            local_indices: &mut Vec<u32>,
                            local_normals: &mut Vec<[f32; 3]>,
                            local_uvs: &mut Vec<[f32; 2]>,
                            indices_offset: &mut u32,
                            uv_coords: UvCoords| {
        local_vertices.extend(vec![
            [-0.5, 0.5, -0.5],  // D 08 Left [0]
            [-0.5, -0.5, -0.5], // A 09 Left [1]
            [-0.5, -0.5, 0.5],  // E 10 Left [2]
            [-0.5, 0.5, 0.5],   // H 11 Left [3]
        ]);

        // 11, 8, 9, 9, 10, 11,
        local_indices.extend([3, 0, 1, 1, 2, 3].iter().map(|x| x + *indices_offset));
        *indices_offset += 4;

        local_normals.extend(vec![
            // Left face (-X)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
        ]);

        render_uvs(local_uvs, uv_coords);
    };

    let should_render_right_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(1, 0, 0);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    let render_right_face = |local_vertices: &mut Vec<[f32; 3]>,
                             local_indices: &mut Vec<u32>,
                             local_normals: &mut Vec<[f32; 3]>,
                             local_uvs: &mut Vec<[f32; 2]>,
                             indices_offset: &mut u32,
                             uv_coords: UvCoords| {
        local_vertices.extend(vec![
            [0.5, -0.5, -0.5], // B 12 Right [0]
            [0.5, 0.5, -0.5],  // C 13 Right [1]
            [0.5, 0.5, 0.5],   // G 14 Right [2]
            [0.5, -0.5, 0.5],  // F 15 Right [3]
        ]);

        // 12, 13, 14, 14, 15, 12
        local_indices.extend([0, 1, 2, 2, 3, 0].iter().map(|x| x + *indices_offset));
        *indices_offset += 4;

        local_normals.extend(vec![
            // Right face (+X)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
        ]);

        render_uvs(local_uvs, uv_coords);
    };

    let should_render_bottom_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(0, -1, 0);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    let render_bottom_face = |local_vertices: &mut Vec<[f32; 3]>,
                              local_indices: &mut Vec<u32>,
                              local_normals: &mut Vec<[f32; 3]>,
                              local_uvs: &mut Vec<[f32; 2]>,
                              indices_offset: &mut u32,
                              uv_coords: UvCoords| {
        local_vertices.extend(vec![
            [-0.5, -0.5, -0.5], // A 16 Bottom [0]
            [0.5, -0.5, -0.5],  // B 17 Bottom [1]
            [0.5, -0.5, 0.5],   // F 18 Bottom [2]
            [-0.5, -0.5, 0.5],  // E 19 Bottom [3]
        ]);

        // 16, 17, 18, 18, 19, 16,
        local_indices.extend([0, 1, 2, 2, 3, 0].iter().map(|x| x + *indices_offset));
        *indices_offset += 4;

        local_normals.extend(vec![
            // Bottom face (-Y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
        ]);

        render_uvs(local_uvs, uv_coords);
    };

    let should_render_top_face = |global_block_pos: &IVec3| -> bool {
        let offset = IVec3::new(0, 1, 0);
        world_map
            .get_block_by_coordinates(&(*global_block_pos + offset))
            .is_none()
    };

    let render_top_face = |local_vertices: &mut Vec<[f32; 3]>,
                           local_indices: &mut Vec<u32>,
                           local_normals: &mut Vec<[f32; 3]>,
                           local_uvs: &mut Vec<[f32; 2]>,
                           indices_offset: &mut u32,
                           uv_coords: UvCoords| {
        local_vertices.extend(vec![
            [0.5, 0.5, -0.5],  // C 20 Top [0]
            [-0.5, 0.5, -0.5], // D 21 Top [1]
            [-0.5, 0.5, 0.5],  // H 22 Top [2]
            [0.5, 0.5, 0.5],   // G 23 Top [3]
        ]);

        // 20, 21, 22, 22, 23, 20,
        local_indices.extend([0, 1, 2, 2, 3, 0].iter().map(|x| x + *indices_offset));
        *indices_offset += 4;

        local_normals.extend(vec![
            // Top face (+Y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
        ]);

        render_uvs(local_uvs, uv_coords);
    };

    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for z in 0..CHUNK_SIZE {
                let local_block_pos = IVec3::new(x, y, z);

                let x = x as f32;
                let y = y as f32;
                let z = z as f32;

                let global_block_pos = &to_global_pos(chunk_pos, &local_block_pos);

                let block = world_map.get_block_by_coordinates(global_block_pos);

                if block.is_none() {
                    continue;
                }

                if crate::world::generation::is_block_surrounded(
                    world_map,
                    chunk_pos,
                    &local_block_pos,
                ) {
                    continue;
                }

                let mut local_vertices: Vec<[f32; 3]> = vec![];
                let mut local_indices: Vec<u32> = vec![];
                let mut local_normals: Vec<[f32; 3]> = vec![];
                let mut local_uvs: Vec<[f32; 2]> = vec![];

                let uv_coords = get_uv_coords(block.unwrap(), registry);

                if should_render_front_face(global_block_pos) {
                    render_front_face(
                        &mut local_vertices,
                        &mut local_indices,
                        &mut local_normals,
                        &mut local_uvs,
                        &mut indices_offset,
                        uv_coords,
                    );
                }

                if should_render_back_face(global_block_pos) {
                    render_back_face(
                        &mut local_vertices,
                        &mut local_indices,
                        &mut local_normals,
                        &mut local_uvs,
                        &mut indices_offset,
                        uv_coords,
                    );
                }

                if should_render_left_face(global_block_pos) {
                    render_left_face(
                        &mut local_vertices,
                        &mut local_indices,
                        &mut local_normals,
                        &mut local_uvs,
                        &mut indices_offset,
                        uv_coords,
                    );
                }

                if should_render_right_face(global_block_pos) {
                    render_right_face(
                        &mut local_vertices,
                        &mut local_indices,
                        &mut local_normals,
                        &mut local_uvs,
                        &mut indices_offset,
                        uv_coords,
                    );
                }

                if should_render_bottom_face(global_block_pos) {
                    render_bottom_face(
                        &mut local_vertices,
                        &mut local_indices,
                        &mut local_normals,
                        &mut local_uvs,
                        &mut indices_offset,
                        uv_coords,
                    );
                }

                if should_render_top_face(global_block_pos) {
                    render_top_face(
                        &mut local_vertices,
                        &mut local_indices,
                        &mut local_normals,
                        &mut local_uvs,
                        &mut indices_offset,
                        uv_coords,
                    );
                }

                let local_vertices: Vec<[f32; 3]> = local_vertices
                    .iter()
                    .map(|v| [v[0] + x + 0.5, v[1] + y + 0.5, v[2] + z + 0.5])
                    .collect();

                vertices.extend(local_vertices);
                indices.extend(local_indices);
                normals.extend(local_normals);
                uvs.extend(local_uvs);
            }
        }
    }

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, default());
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices.to_vec());
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
