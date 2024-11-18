use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use shared::{world::*, CHUNK_SIZE};
use std::collections::HashMap;

pub fn generate_chunk(chunk_pos: IVec3, seed: u32) -> ServerChunk {
    let perlin = Perlin::new(seed);

    let scale = 0.1;
    let max_perlin_height_variation = 5.0;
    let base_height = 32; // should be 64

    let cx = chunk_pos.x;
    let cy = chunk_pos.y;
    let cz = chunk_pos.z;

    let mut chunk = ServerChunk {
        map: HashMap::new(),
        ts: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    };

    for dx in 0..CHUNK_SIZE {
        for dz in 0..CHUNK_SIZE {
            let x = CHUNK_SIZE * cx + dx;
            let z = CHUNK_SIZE * cz + dz;

            let perlin_height = (perlin.get([x as f64 * scale, z as f64 * scale]) - 0.5)
                * max_perlin_height_variation;

            let terrain_height = base_height + perlin_height.round() as i32;

            for dy in 0..CHUNK_SIZE {
                let y = CHUNK_SIZE * cy + dy;

                if y > terrain_height {
                    break;
                }

                let block = if y == 0 {
                    BlockId::Bedrock
                } else if y < terrain_height - 2 {
                    BlockId::Stone
                } else if y < terrain_height {
                    BlockId::Dirt
                } else {
                    BlockId::Grass
                };

                chunk
                    .map
                    .insert(IVec3::new(dx, dy, dz), BlockData::new(block));
            }
        }
    }
    chunk
}

// pub fn setup_world(
//     mut commands: Commands,
//     mut world_map: ResMut<ServerWorldMap>,
//     r_blocks: Res<Registry<BlockData>>,
// ) {
//     debug!("Registry : {:?}", r_blocks);
//     let seed = random::<u32>();
//     commands.insert_resource(WorldSeed(seed));
//
//     debug!("Generating a new world with seed: {}", seed);
//
//     for x in -1..=1 {
//         for y in 0..=8 {
//             for z in -1..=1 {
//                 let pos = IVec3::new(x, y, z);
//
//                 let chunk = generate_chunk(pos, seed, &r_blocks);
//                 world_map.map.insert(pos, chunk);
//                 world_map.chunks_to_update.push(pos);
//             }
//         }
//     }
// }
