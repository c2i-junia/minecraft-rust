use crate::constants::CHUNK_SIZE;
use crate::world::block_to_chunk_coord;
use bevy::math::IVec3;

pub fn to_global_pos(chunk_pos: &IVec3, local_block_pos: &IVec3) -> IVec3 {
    *chunk_pos * CHUNK_SIZE + *local_block_pos
}

pub fn global_block_to_chunk_pos(global_block_pos: &IVec3) -> IVec3 {
    IVec3::new(
        block_to_chunk_coord(global_block_pos.x),
        block_to_chunk_coord(global_block_pos.y),
        block_to_chunk_coord(global_block_pos.z),
    )
}
