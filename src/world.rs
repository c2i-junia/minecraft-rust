use crate::materials::MaterialResource;
use crate::BlockRaycastSet;
use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use noise::{NoiseFn, Perlin};
use std::collections::HashMap;

#[derive(Component)]
pub struct BlockMarker;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Block {
    Grass,
    Dirt,
}

#[derive(Resource, Default)]
pub struct WorldMap {
    pub map: HashMap<IVec3, HashMap<IVec3, Block>>,
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
        chunk.insert(IVec3::new(sub_x, sub_y, sub_z), block);

        let material = material_resource
            .materials
            .get(&block)
            .expect("material not found")
            .clone();

        commands.spawn((
            BlockMarker,
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec3::new(x as f32, y as f32, z as f32)),
                ..Default::default()
            },
            RaycastMesh::<BlockRaycastSet>::default(), // Permet aux rayons de détecter ces blocs
        ));
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

    let max_perlin_height = 10.0;

    let cx = chunk_pos.x;
    let cz = chunk_pos.z;

    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));

    // Boucle pour générer les blocs avec variation de hauteur
    for i in 0..16 {
        for j in 0..16 {
            let x = 16 * cx + i;
            let z = 16 * cz + j;
            // Générer une hauteur en utilisant le bruit de Perlin
            let perlin_height =
                perlin.get([x as f64 * scale, z as f64 * scale]) * max_perlin_height;
            let perlin_height = perlin_height.round() as i32; // Arrondir à des hauteurs entières

            // Générer les couches de blocs jusqu'à la couche y = -10
            for y in -10..=perlin_height {
                let block = if y == perlin_height {
                    Block::Grass
                } else {
                    Block::Dirt
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
                // Placer chaque bloc à la bonne hauteur
                // Marquer les blocs comme détectables par raycasting

                world_map.total_blocks_count += 1;
            }
        }
    }

    world_map.total_chunks_count += 1;

    /*
    println!(
        "Total block count {}",
        WORLD_MAP.lock().unwrap().total_blocks_count
    );
     */
}

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut world_map: ResMut<WorldMap>,
    material_resource: Res<MaterialResource>,
) {
    for x in -1..=1 {
        for z in -1..=1 {
            generate_chunk(
                IVec3::new(x, 0, z),
                42,
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
                42,
                commands,
                meshes,
                world_map,
                &material_resource,
            );
        }
    }
}
