mod auth;
mod chat;
mod player;
mod system;
mod world;

use crate::world::BlockId;
pub use auth::*;
use bevy::math::IVec3;
pub use chat::*;
pub use player::*;
use serde::{Deserialize, Serialize};
pub use system::*;
pub use world::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ClientToServerMessage {
    AuthRegisterRequest(AuthRegisterRequest),
    ChatMessage(ChatMessage),
    ShutdownOrder(ShutdownOrder),
    PlayerInputs(PlayerInputs),
    WorldUpdateRequest {
        player_chunk_position: IVec3,
        render_distance: u32,
        requested_chunks: Vec<IVec3>,
    },
    SaveWorldRequest(SaveWorldRequest),
    BlockInteraction {
        position: IVec3,
        block_type: Option<BlockId>,
    },
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum ServerToClientMessage {
    AuthRegisterResponse(AuthRegisterResponse),
    ChatConversation(ChatConversation),
    WorldUpdate(WorldUpdate),
}
