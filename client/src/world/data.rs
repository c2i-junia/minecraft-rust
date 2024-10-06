use crate::constants::CHUNK_SIZE;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use shared::world::{block_to_chunk_coord, global_block_to_chunk_pos, to_local_pos};
use std::collections::HashMap;

pub type Block = shared::world::Block;

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
    pub(crate) entity: Option<Entity>,
}

#[derive(Resource, Default, Clone, Serialize, Deserialize)]
pub struct WorldMap {
    pub name: String,
    pub map: HashMap<IVec3, Chunk>,
    pub total_blocks_count: u64,
    pub total_chunks_count: u64,
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
