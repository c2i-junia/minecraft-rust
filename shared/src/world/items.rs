use std::{fmt::Debug, mem::transmute};

use serde::{Deserialize, Serialize};

use super::{BlockId, GameElementId};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    Hash,
    Default,
)]
#[repr(usize)]
pub enum ItemId {
    #[default]
    Dirt,
    // ! ----- LEAVE DIRT FIRST ----- !
    Grass,
    Stone,
    OakLog,
    OakPlanks,
    Sand,
    // ! ----- LEAVE BEDROCK LAST ----- !
    Bedrock,
}

impl ItemId {
    pub fn get_max_stack(&self) -> u32 {
        64
    }

    pub fn get_default_type(&self) -> ItemType {
        match *self {
            Self::Dirt => ItemType::Block(BlockId::Dirt),
            Self::Bedrock => ItemType::Block(BlockId::Bedrock),
            Self::Grass => ItemType::Block(BlockId::Grass),
            Self::Stone => ItemType::Block(BlockId::Stone),
            Self::OakLog => ItemType::Block(BlockId::OakLog),
            Self::OakPlanks => ItemType::Block(BlockId::OakPlanks),
            Self::Sand => ItemType::Block(BlockId::Sand),
        }
    }
}

impl GameElementId for ItemId {
    fn iterate_enum() -> impl Iterator<Item = ItemId> {
        // Unsafe code needed for `transmute` function
        // Transmute function needed to cast from `usize` to `ItemId`
        // Still safe, because `ItemId` enum only contains numerical enum variants
        unsafe { ((Self::Dirt as usize)..=(Self::Bedrock as usize)).map(|num| transmute(num)) }
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
    Generic,
    Block(BlockId),
    Tool { durability: i16 },
    Armor(ArmorType),
}
