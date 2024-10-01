pub mod messages;

use bevy::prelude::*;
use bevy_renet::renet::{ClientId, ConnectionConfig};
use serde::{Deserialize, Serialize};

pub const PRIVATE_KEY: &[u8; bevy_renet::renet::transport::NETCODE_KEY_BYTES] =
    b"an example very very secret key."; // 32-bytes
pub const PROTOCOL_ID: u64 = 0;

#[derive(Debug, Component)]
pub struct Player {
    pub id: ClientId,
}

#[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component, Resource)]
pub struct PlayerInput {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
}

#[derive(Debug, Serialize, Deserialize, Event)]
pub enum PlayerCommand {
    BasicAttack { cast_at: Vec3 },
}

#[derive(Debug, Default, Component)]
pub struct Velocity(pub Vec3);

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessages {
    ChatMessage { message: String },
}

pub fn connection_config() -> ConnectionConfig {
    ConnectionConfig {
        available_bytes_per_tick: 1024 * 1024,
        server_channels_config: default(),
        client_channels_config: default(),
    }
}
