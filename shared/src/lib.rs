use bevy::prelude::Resource;

pub mod messages;
pub mod world;

#[derive(Resource)]
pub struct GameServerConfig {
    pub world_name: String,
    pub is_solo: bool,
}

pub const PROTOCOL_ID: u64 = 0;
pub const CHUNK_SIZE: i32 = 16;
