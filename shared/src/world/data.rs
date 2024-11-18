use crate::world::block_to_chunk_coord;
use crate::world::global_block_to_chunk_pos;
use crate::world::to_local_pos;
use crate::CHUNK_SIZE;
use bevy::math::IVec3;
use bevy::prelude::Resource;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::BlockData;
use super::ItemId;
use super::ItemType;

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
pub struct ServerChunk {
    pub map: HashMap<IVec3, BlockData>,
    /// Timestamp marking the last update this chunk has received
    pub ts: u64,
}

#[derive(Resource, Default, Clone, Serialize, Deserialize, Debug)]
pub struct ServerWorldMap {
    pub name: String,
    pub map: HashMap<IVec3, ServerChunk>,
    pub chunks_to_update: Vec<IVec3>,
}

#[derive(Resource, Serialize, Deserialize)]
pub struct WorldSeed(pub u32);

#[derive(Debug, Clone, Serialize, Deserialize, Copy)]
pub struct ItemStack {
    pub item_id: ItemId,
    pub item_type: ItemType,
    pub nb: u32,
}

impl ServerWorldMap {
    pub fn get_block_by_coordinates(&self, position: &IVec3) -> Option<&BlockData> {
        let x: i32 = position.x;
        let y: i32 = position.y;
        let z: i32 = position.z;
        let cx: i32 = block_to_chunk_coord(x);
        let cy: i32 = block_to_chunk_coord(y);
        let cz: i32 = block_to_chunk_coord(z);
        let chunk: Option<&ServerChunk> = self.map.get(&IVec3::new(cx, cy, cz));
        match chunk {
            Some(chunk) => {
                let sub_x: i32 = ((x % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
                let sub_y: i32 = ((y % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
                let sub_z: i32 = ((z % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
                chunk.map.get(&IVec3::new(sub_x, sub_y, sub_z))
            }
            None => None,
        }
    }

    pub fn remove_block_by_coordinates(&mut self, global_block_pos: &IVec3) -> Option<BlockData> {
        let block: &BlockData = self.get_block_by_coordinates(global_block_pos)?;
        let kind: BlockData = *block;

        let chunk_pos: IVec3 = global_block_to_chunk_pos(global_block_pos);

        let chunk_map: &mut ServerChunk =
            self.map
                .get_mut(&IVec3::new(chunk_pos.x, chunk_pos.y, chunk_pos.z))?;

        let local_block_pos: IVec3 = to_local_pos(global_block_pos);

        chunk_map.map.remove(&local_block_pos);

        Some(kind)
    }

    pub fn set_block(&mut self, position: &IVec3, block: BlockData) {
        let x: i32 = position.x;
        let y: i32 = position.y;
        let z: i32 = position.z;
        let cx: i32 = block_to_chunk_coord(x);
        let cy: i32 = block_to_chunk_coord(y);
        let cz: i32 = block_to_chunk_coord(z);
        let chunk: &mut ServerChunk = self.map.entry(IVec3::new(cx, cy, cz)).or_default();
        let sub_x: i32 = ((x % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
        let sub_y: i32 = ((y % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;
        let sub_z: i32 = ((z % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE;

        chunk.map.insert(IVec3::new(sub_x, sub_y, sub_z), block);
    }
}
