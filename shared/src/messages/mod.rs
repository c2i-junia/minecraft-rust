mod auth;
mod chat;
pub mod player;
mod system;
mod world;

use crate::world::BlockData;
pub use auth::*;
use bevy::math::{IVec3, Vec3};
pub use chat::*;
pub use player::*;
use serde::{Deserialize, Serialize};
pub use system::*;
pub use world::*;

pub type PlayerId = u64;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ClientToServerMessage {
    AuthRegisterRequest(AuthRegisterRequest),
    ChatMessage(ChatMessage),
    Exit(ExitOrder),
    PlayerInputs(PlayerInputs),
    WorldUpdateRequest {
        player_chunk_position: IVec3,
        render_distance: u32,
        requested_chunks: Vec<IVec3>,
    },
    SaveWorldRequest(SaveWorldRequest),
    BlockInteraction {
        position: IVec3,
        block_type: Option<BlockData>,
    },
    SetPlayerPosition {
        // should be deprecated in the long run
        position: Vec3,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ServerToClientMessage {
    AuthRegisterResponse(AuthRegisterResponse),
    ChatConversation(ChatConversation),
    WorldUpdate(WorldUpdate),
    PlayerSpawn(PlayerSpawnEvent),
}
