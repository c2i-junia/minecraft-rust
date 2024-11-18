use std::fmt::Debug;

use serde::{Deserialize, Serialize};

use super::BlockId;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
)]
pub enum ItemId {
    Bedrock,
    Dirt,
    Grass,
    Stone,
}

impl ItemId {
    pub fn get_max_stack(&self) -> u32 {
        64
    }
}

/// Data associated with a given `BlockId`
#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BlockData {
    pub id: BlockId,
}

impl BlockData {
    pub fn new(id: BlockId) -> Self {
        BlockData { id }
    }
}

/// Temporary struct for deserialization purposes
#[derive(Debug, Serialize, Deserialize)]
pub struct TempBlock {
    pub id: String,
    pub drops: Vec<(u16, String)>,
    pub break_time: f32,
    pub uvs: [f32; 4],
}

/// Data associated with a given `ItemId`
#[derive(Debug, Clone, Copy)]
pub struct ItemData {
    pub kind: ItemType,
    pub stack: u8,
}

/// Type of armor piece
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ArmorType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
}

/// Type of item
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ItemType {
    Block(BlockId),
    Tool,
    Armor(ArmorType),
}

impl Default for ItemData {
    fn default() -> Self {
        Self {
            kind: ItemType::Tool,
            stack: 64,
        }
    }
}
