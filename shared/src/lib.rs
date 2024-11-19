use bevy::prelude::Resource;
use bevy_renet::renet::ConnectionConfig;

pub mod messages;
pub mod world;

#[derive(Resource)]
pub struct GameServerConfig {
    pub world_name: String,
    pub is_solo: bool,
}

pub const PROTOCOL_ID: u64 = 0;
pub const CHUNK_SIZE: i32 = 16;

pub fn get_shared_renet_config() -> ConnectionConfig {
    Default::default()
}
