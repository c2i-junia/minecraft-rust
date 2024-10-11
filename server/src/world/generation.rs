use crate::world::data::*;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::random;
use shared::world::*;
use std::collections::HashMap;

fn generate_chunk(chunk_pos: IVec3, seed: u32, r_items: &Registry<ItemData>) -> Chunk {
    let perlin = Perlin::new(seed);

    let scale = 0.1;
    let max_perlin_height_variation = 5.0;
    let base_height = 32; // should be 64

    const WORLD_MIN_Y: i32 = 0;

    let cx = chunk_pos.x;
    let cz = chunk_pos.z;

    let mut chunk = Chunk {
        map: HashMap::new(),
    };

    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            let x = CHUNK_SIZE * cx + i;
            let z = CHUNK_SIZE * cz + j;

            let perlin_height = (perlin.get([x as f64 * scale, z as f64 * scale]) - 0.5)
                * max_perlin_height_variation;

            let terrain_height = base_height + perlin_height.round() as i32;

            for y in WORLD_MIN_Y..=terrain_height {
                let block = if y == 0 {
                    BlockType::Bedrock.get_name()
                } else if y < terrain_height - 2 {
                    BlockType::Stone.get_name()
                } else if y < terrain_height {
                    BlockType::Dirt.get_name()
                } else {
                    BlockType::Grass.get_name()
                };

                chunk
                    .map
                    .insert(IVec3::new(x, y, z), *r_items.get_id(&block).unwrap());
            }
        }
    }

    chunk
}

pub fn setup_world(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    r_items: Res<Registry<ItemData>>,
) {
    println!("Registry : {:?}", r_items);
    let seed = random::<u32>();
    commands.insert_resource(WorldSeed(seed));

    println!("Generating a new world with seed: {}", seed);

    for x in -1..=1 {
        for y in 0..=8 {
            for z in -1..=1 {
                let pos = IVec3::new(x, y, z);
                let chunk = generate_chunk(pos, seed, &r_items);
                world_map.map.insert(pos, chunk);
            }
        }
    }
}
