use crate::constants::CHUNK_SIZE;
use crate::world::block_to_chunk_coord;
use bevy::math::IVec3;

pub fn to_global_pos(chunk_pos: &IVec3, local_block_pos: &IVec3) -> IVec3 {
    *chunk_pos * CHUNK_SIZE + *local_block_pos
}

pub fn to_local_pos(global_block_pos: &IVec3) -> IVec3 {
    IVec3 {
        x: ((global_block_pos.x % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
        y: ((global_block_pos.y % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
        z: ((global_block_pos.z % CHUNK_SIZE) + CHUNK_SIZE) % CHUNK_SIZE,
    }
}

pub fn global_block_to_chunk_pos(global_block_pos: &IVec3) -> IVec3 {
    IVec3::new(
        block_to_chunk_coord(global_block_pos.x),
        block_to_chunk_coord(global_block_pos.y),
        block_to_chunk_coord(global_block_pos.z),
    )
}

pub const SIX_OFFSETS: [IVec3; 6] = [
    IVec3::new(1, 0, 0),
    IVec3::new(-1, 0, 0),
    IVec3::new(0, 1, 0),
    IVec3::new(0, -1, 0),
    IVec3::new(0, 0, 1),
    IVec3::new(0, 0, -1),
];
