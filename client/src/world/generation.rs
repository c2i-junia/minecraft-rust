use crate::constants::CHUNK_SIZE;
use crate::world::utils::{global_block_to_chunk_pos, to_global_pos, to_local_pos, SIX_OFFSETS};
use crate::BlockRaycastSet;
use crate::MaterialResource;
use bevy::prelude::Resource;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_mod_raycast::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::collections::HashMap;

use crate::world::*;
use serde::{Deserialize, Serialize};

use super::RenderDistance;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Block {
    Grass,
    Dirt,
    Stone,
    Bedrock,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum GlobalMaterial {
    Sun,
    Moon,
}

#[derive(Component, Clone, Serialize, Deserialize)]
pub struct BlockWrapper {
    pub kind: Block,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct WorldSeed(pub u32);

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Chunk {
    map: HashMap<IVec3, BlockWrapper>,
    #[serde(skip)]
    entity: Option<Entity>,
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub map: HashMap<IVec3, Chunk>,
    pub total_blocks_count: u64,
    pub total_chunks_count: u64,
}

pub fn block_to_chunk_coord(x: i32) -> i32 {
    if x >= 0 {
        x / CHUNK_SIZE
    } else {
        (x - (CHUNK_SIZE - 1)) / CHUNK_SIZE
    }
}

pub fn block_vec3_to_chunk_v3_coord(v: Vec3) -> Vec3 {
    Vec3::new(
        block_to_chunk_coord(v.x as i32) as f32,
        block_to_chunk_coord(v.y as i32) as f32,
        block_to_chunk_coord(v.z as i32) as f32,
    )
}

impl WorldMap {
    pub fn get_block_by_coordinates(&self, position: &IVec3) -> Option<&BlockWrapper> {
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let cx = block_to_chunk_coord(x);
        let cy = block_to_chunk_coord(y);
        let cz = block_to_chunk_coord(z);
        let chunk = self.map.get(&IVec3::new(cx, cy, cz));
        match chunk {
            Some(chunk) => {
                let sub_x = ((x % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
                let sub_y = ((y % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
                let sub_z = ((z % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
                chunk.map.get(&IVec3::new(sub_x, sub_y, sub_z))
            }
            None => None,
        }
    }

    pub fn remove_block_by_coordinates(&mut self, global_block_pos: &IVec3) -> Option<Block> {
        let block = self.get_block_by_coordinates(global_block_pos)?;
        let kind = block.kind;

        let chunk_pos = global_block_to_chunk_pos(global_block_pos);

        let chunk_map = self
            .map
            .get_mut(&IVec3::new(chunk_pos.x, chunk_pos.y, chunk_pos.z))?;

        let local_block_pos = to_local_pos(global_block_pos);

        chunk_map.map.remove(&local_block_pos);

        Some(kind)
    }

    pub fn set_block(&mut self, position: &IVec3, block: Block) {
        let x = position.x;
        let y = position.y;
        let z = position.z;
        let cx = block_to_chunk_coord(x);
        let cy = block_to_chunk_coord(y);
        let cz = block_to_chunk_coord(z);
        let chunk = self.map.entry(IVec3::new(cx, cy, cz)).or_default();
        let sub_x = ((x % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
        let sub_y = ((y % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
        let sub_z = ((z % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;

        if x == 0 && z == 0 {
            // println!("inserting y={}", y)
        }
        chunk.map.insert(
            IVec3::new(sub_x, sub_y, sub_z),
            BlockWrapper { kind: block },
        );
    }
}

fn generate_chunk(
    chunk_pos: IVec3,
    seed: u32,
    world_map: &mut WorldMap,
    ev_render: &mut EventWriter<WorldRenderRequestUpdateEvent>,
) {
    //println!("gen chunk {}", chunk_pos);
    let perlin = Perlin::new(seed);

    let scale = 0.1;
    let max_perlin_height_variation = 5.0;
    let base_height = 32; // should be 64

    const WORLD_MIN_Y: i32 = 0;

    let cx = chunk_pos.x;
    let cz = chunk_pos.z;

    // Boucle pour générer les blocs avec variation de hauteur
    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            let x = CHUNK_SIZE * cx + i;
            let z = CHUNK_SIZE * cz + j;

            // Générer une hauteur en utilisant le bruit de Perlin
            let perlin_height = (perlin.get([x as f64 * scale, z as f64 * scale]) - 0.5)
                * max_perlin_height_variation;

            // Ajouter un offset de 64 blocs pour centrer la hauteur autour de y = 64
            let terrain_height = base_height + perlin_height.round() as i32;

            // Générer des blocs à partir de la couche 0 (bedrock) jusqu'à la hauteur générée
            for y in WORLD_MIN_Y..=terrain_height {
                let block = if y == 0 {
                    Block::Bedrock // Placer la bedrock à la couche 0
                } else if y < terrain_height - 2 {
                    Block::Stone // Placer de la pierre en dessous des 3 dernières couches
                } else if y < terrain_height {
                    Block::Dirt // Placer de la terre dans les 3 couches sous la surface
                } else {
                    Block::Grass // Placer de l'herbe à la surface
                };

                world_map.set_block(&IVec3::new(x, y, z), block);

                // Incrémenter le compteur de blocs
                world_map.total_blocks_count += 1;
            }
        }
    }

    world_map.total_chunks_count += 1;
    for y in 0..=3 {
        let mut pos = chunk_pos.clone();
        pos.y = y;
        ev_render.send(WorldRenderRequestUpdateEvent::ChunkToReload(pos));
    }
    // println!("sending event for {}", chunk_pos);
}

pub fn setup_world(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    mut ev_render: EventWriter<WorldRenderRequestUpdateEvent>,
) {
    // Charger la graine depuis le fichier `world_seed.ron`
    let seed = match load_world_seed("world_seed.ron") {
        Ok(seed) => {
            println!("Loaded existing world seed from world_seed.ron");
            seed.0
        }
        Err(_) => {
            // Si la graine n'est pas trouvée, en générer une nouvelle
            let seed = rand::thread_rng().gen::<u32>();
            println!("Generated random seed: {}", seed);
            seed
        }
    };

    commands.insert_resource(WorldSeed(seed));

    // Charger la carte du monde depuis le fichier `world_save.ron`
    if let Ok(loaded_world) = load_world_map("world_save.ron") {
        *world_map = loaded_world;
        println!("Loaded existing world from world_save.ron");

        // we need to recreate the entities because their are not
        // saved in the world_save file
        for (chunk_pos, chunk) in world_map.map.iter_mut() {
            if chunk.entity.is_none() {
                let new_entity = commands
                    .spawn((
                        Transform::from_xyz(
                            (chunk_pos.x * CHUNK_SIZE) as f32,
                            (chunk_pos.y * CHUNK_SIZE) as f32,
                            (chunk_pos.z * CHUNK_SIZE) as f32,
                        ),
                        GlobalTransform::default(),
                    ))
                    .id();
                chunk.entity = Some(new_entity);
            }

            // now that the entities are loaded, we need to send events to
            // update the rendering
            for x in -1..=1 {
                for z in -1..=1 {
                    ev_render.send(WorldRenderRequestUpdateEvent::BlockToReload(IVec3::new(
                        x, 0, z,
                    )));
                }
            }
        }
    } else {
        // Si le chargement échoue, on génère un nouveau monde
        println!("Generating a new world with seed: {}", seed);

        for x in -1..=1 {
            for y in 0..=8 {
                for z in -1..=1 {
                    generate_chunk(IVec3::new(x, y, z), seed, &mut world_map, &mut ev_render);
                }
            }
        }
    }
}

pub fn load_chunk_around_player(
    player_position: Vec3,
    world_map: &mut WorldMap,
    seed: u32,
    ev_render: &mut EventWriter<WorldRenderRequestUpdateEvent>,
    render_distance: Res<RenderDistance>,
) {
    let player_chunk = IVec3::new(
        block_to_chunk_coord(player_position.x as i32),
        0,
        block_to_chunk_coord(player_position.z as i32),
    );

    let r = render_distance.distance as i32;

    for x in -r..=r {
        for z in -r..=r {
            let chunk_pos = IVec3::new(player_chunk.x + x, 0, player_chunk.z + z);
            {
                let chunk = world_map.map.get(&chunk_pos);
                if chunk.is_some() {
                    continue;
                }
                // Doing these scoping shenanigans to release the Mutex at the end of the scope
                // because generate_chunk requires a Mutex lock as well
            }
            generate_chunk(chunk_pos, seed, world_map, ev_render);
        }
    }
}

#[derive(Event, Debug, Copy, Clone)]
pub enum WorldRenderRequestUpdateEvent {
    ChunkToReload(IVec3),
    BlockToReload(IVec3),
}

fn is_block_surrounded(world_map: &WorldMap, chunk_pos: &IVec3, local_block_pos: &IVec3) -> bool {
    let global_block_pos = to_global_pos(chunk_pos, local_block_pos);

    for offset in &SIX_OFFSETS {
        let neighbor_pos = global_block_pos + *offset;

        // Check if the block exists at the neighboring position
        if world_map.get_block_by_coordinates(&neighbor_pos).is_none() {
            return false;
        }
    }

    true
}

#[derive(Copy, Clone)]
struct UvCoords {
    u0: f32,
    u1: f32,
    v0: f32,
    v1: f32,
}

fn get_uv_coords(block: Block) -> UvCoords {
    // should be refactored later
    let res = match block {
        Block::Grass => [0.0, 0.25, 0.0, 1.0],
        Block::Dirt => [0.25, 0.5, 0.0, 1.0],
        Block::Stone => [0.5, 0.75, 0.0, 1.0],
        Block::Bedrock => [0.75, 1.0, 0.0, 1.0],
    };
    UvCoords {
        u0: res[0],
        u1: res[1],
        v0: res[2],
        v1: res[3],
    }
}

fn generate_chunk_mesh(world_map: &WorldMap, chunk_pos: &IVec3) -> Mesh {
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
        local_indices.extend(vec![0, 3, 2, 2, 1, 0].iter().map(|x| x + *indices_offset));
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
        local_indices.extend(vec![0, 1, 2, 2, 3, 0].iter().map(|x| x + *indices_offset));
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
        local_indices.extend(vec![3, 0, 1, 1, 2, 3].iter().map(|x| x + *indices_offset));
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
        local_indices.extend(vec![0, 1, 2, 2, 3, 0].iter().map(|x| x + *indices_offset));
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
        local_indices.extend(vec![0, 1, 2, 2, 3, 0].iter().map(|x| x + *indices_offset));
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
        local_indices.extend(vec![0, 1, 2, 2, 3, 0].iter().map(|x| x + *indices_offset));
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

                if is_block_surrounded(world_map, chunk_pos, &local_block_pos) {
                    continue;
                }

                let mut local_vertices: Vec<[f32; 3]> = vec![];
                let mut local_indices: Vec<u32> = vec![];
                let mut local_normals: Vec<[f32; 3]> = vec![];
                let mut local_uvs: Vec<[f32; 2]> = vec![];

                let uv_coords = get_uv_coords(block.unwrap().kind);

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

#[derive(Default)]
pub struct QueuedEvents {
    pub events: Vec<WorldRenderRequestUpdateEvent>,
}

pub fn world_render_system(
    mut world_map: ResMut<WorldMap>,
    material_resource: Res<MaterialResource>,
    mut commands: Commands,
    mut ev_render: EventReader<WorldRenderRequestUpdateEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut queued_events: Local<QueuedEvents>,
) {
    for event in ev_render.read() {
        queued_events.events.push(event.clone());
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
        for (chunk_pos, chunk) in cloned_map.map.iter_mut() {
            if !chunks_pos_to_reload.contains(chunk_pos) {
                continue;
            }

            update_chunk(
                chunk,
                chunk_pos,
                &material_resource,
                &mut commands,
                &mut meshes,
                &mut world_map,
            );
        }
    }
    queued_events.events.clear();
}

fn update_chunk(
    chunk: &mut Chunk,
    chunk_pos: &IVec3,
    material_resource: &Res<MaterialResource>,
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    world_map: &mut WorldMap,
) {
    //println!("update_chunk {}", chunk_pos);
    let texture = material_resource.atlas_texture.clone().unwrap();
    let new_mesh = generate_chunk_mesh(world_map, chunk_pos);

    if chunk.entity.is_some() {
        commands
            .get_entity(chunk.entity.unwrap())
            .unwrap()
            .despawn_recursive();
        chunk.entity = None;
    }

    if chunk.entity.is_none() {
        //println!("update_chunk {}", chunk_pos);
        // Cube
        let new_entity = commands
            .spawn((
                PbrBundle {
                    mesh: meshes.add(new_mesh),
                    material: texture.clone(),
                    transform: Transform::from_xyz(
                        (chunk_pos.x * CHUNK_SIZE) as f32,
                        (chunk_pos.y * CHUNK_SIZE) as f32,
                        (chunk_pos.z * CHUNK_SIZE) as f32,
                    ),
                    ..Default::default()
                },
                RaycastMesh::<BlockRaycastSet>::default(),
            ))
            .id();

        let ch = world_map.map.get_mut(chunk_pos).unwrap();
        ch.entity = Some(new_entity);
    }
}
