use crate::constants::CUBE_SIZE;
use crate::materials::MaterialResource;
use crate::BlockRaycastSet;
use bevy::prelude::Resource;
use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use std::collections::HashMap;

#[derive(Component)]
pub struct BlockMarker;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Block {
    Grass,
    Dirt,
    Stone,
    Bedrock,
}

#[derive(Component)]
pub struct BlockWrapper {
    pub kind: Block,
    pub entity: Entity,
}

#[derive(Resource)]
pub struct WorldSeed(pub u32);

#[derive(Resource, Default)]
pub struct WorldMap {
    pub map: HashMap<IVec3, HashMap<IVec3, BlockWrapper>>,
    pub total_blocks_count: u64,
    pub total_chunks_count: u64,
}

pub fn block_to_chunk_coord(x: i32) -> i32 {
    if x >= 0 {
        x / 16
    } else {
        (x - 15) / 16
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
    // pub fn get_block(&self, x: i32, y: i32, z: i32) -> Option<&Block> {
    //     let cx = block_to_chunk_coord(x);
    //     let cy = block_to_chunk_coord(y);
    //     let cz = block_to_chunk_coord(z);
    //     let chunk = self.map.get(&IVec3::new(cx, cy, cz));
    //     match chunk {
    //         Some(chunk) => {
    //             let sub_x = x % 16;
    //             let sub_y = y % 16;
    //             let sub_z = z % 16;
    //             chunk.get(&IVec3::new(sub_x, sub_y, sub_z))
    //         }
    //         None => None,
    //     }
    // }

    pub fn get_block_wrapper_by_entity(&self, entity: Entity) -> Option<&BlockWrapper> {
        for (_, inner_map) in &self.map {
            for (_, value) in inner_map {
                if value.entity == entity {
                    return Some(value);
                }
            }
        }
        None
    }

    pub fn remove_block_by_entity(
        &mut self,
        entity: Entity,
        commands: &mut Commands,
    ) -> Option<Block> {
        let wrapper = self.get_block_wrapper_by_entity(entity)?;
        commands.entity(wrapper.entity).despawn_recursive();
        Some(wrapper.kind)
    }

    pub fn set_block(
        &mut self,
        x: i32,
        y: i32,
        z: i32,
        block: Block,
        commands: &mut Commands,
        mesh: Handle<Mesh>,
        material_resource: &Res<MaterialResource>,
    ) {
        let cx = block_to_chunk_coord(x);
        let cy = block_to_chunk_coord(y);
        let cz = block_to_chunk_coord(z);
        let chunk = self
            .map
            .entry(IVec3::new(cx, cy, cz))
            .or_insert(HashMap::new());
        let sub_x = x % 16;
        let sub_y = y % 16;
        let sub_z = z % 16;

        let material = material_resource
            .materials
            .get(&block)
            .expect("material not found")
            .clone();

        let entity = commands
            .spawn((
                BlockMarker,
                PbrBundle {
                    mesh,
                    material,
                    transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                    ..Default::default()
                },
                RaycastMesh::<BlockRaycastSet>::default(), // Permet aux rayons de détecter ces blocs
            ))
            .id();

        chunk.insert(
            IVec3::new(sub_x, sub_y, sub_z),
            BlockWrapper {
                kind: block,
                entity,
            },
        );
    }
}

fn generate_chunk(
    chunk_pos: IVec3,
    seed: u32,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    world_map: &mut WorldMap,
    material_resource: &Res<MaterialResource>,
) {
    // println!(
    //     "Generating chunk: {}, {}, {}",
    //     chunk_pos.x, chunk_pos.y, chunk_pos.z
    // );
    let perlin = Perlin::new(seed);

    let scale = 0.1;
    let max_perlin_height_variation = 5.0;
    let base_height = 10; // should be 64

    const CHUNK_SIZE: i32 = 16;
    const WORLD_MIN_Y: i32 = 0;

    let cx = chunk_pos.x;
    let cz = chunk_pos.z;

    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE)));

    // Boucle pour générer les blocs avec variation de hauteur
    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            let x = CHUNK_SIZE * cx + i;
            let z = CHUNK_SIZE * cz + j;

            // Générer une hauteur en utilisant le bruit de Perlin
            let perlin_height =
                perlin.get([x as f64 * scale, z as f64 * scale]) * max_perlin_height_variation;

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

                world_map.set_block(
                    x,
                    y,
                    z,
                    block,
                    commands,
                    cube_mesh.clone(),
                    material_resource,
                );

                // Incrémenter le compteur de blocs
                world_map.total_blocks_count += 1;
            }
        }
    }

    world_map.total_chunks_count += 1;
}

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut world_map: ResMut<WorldMap>,
    material_resource: Res<MaterialResource>,
) {
    let seed = rand::thread_rng().gen::<u32>();
    println!("Generated random seed: {}", seed);

    commands.insert_resource(WorldSeed(seed));

    for x in -1..=1 {
        for z in -1..=1 {
            generate_chunk(
                IVec3::new(x, 0, z),
                seed,
                &mut commands,
                &mut meshes,
                &mut world_map,
                &material_resource,
            );
        }
    }
}

pub fn load_chunk_around_player(
    player_position: Vec3,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    world_map: &mut WorldMap,
    material_resource: Res<MaterialResource>,
    seed: u32,
) {
    let player_chunk = IVec3::new(
        block_to_chunk_coord(player_position.x as i32),
        0,
        block_to_chunk_coord(player_position.z as i32),
    );

    for x in -4..=4 {
        for z in -4..=4 {
            let chunk_pos = IVec3::new(player_chunk.x + x, 0, player_chunk.z + z);
            {
                let chunk = world_map.map.get(&chunk_pos);
                if chunk.is_some() {
                    continue;
                }
                // Doing these scoping shenanigans to release the Mutex at the end of the scope
                // because generate_chunk requires a Mutex lock as well
            }
            generate_chunk(
                chunk_pos,
                seed,
                commands,
                meshes,
                world_map,
                &material_resource,
            );
        }
    }
}
