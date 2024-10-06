use std::fs;
use std::{collections::HashMap, fmt::Debug};

use bevy::reflect::Enum;
use bevy::{
    prelude::{ResMut, Resource},
    scene::ron::from_str,
};
use serde::{Deserialize, Serialize};

pub type BlockId = u32;
pub type ItemId = u32;

pub type Test = &'static dyn Enum;

/// Data associated with a given `BlockId`
#[derive(Debug, Clone)]
pub struct BlockData {
    pub drops: Vec<(u16, ItemId)>,
    pub break_time: f32,
    pub uvs: [f32; 4],
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
#[derive(Debug)]
pub struct ItemData {
    pub kind: ItemType,
    pub stack: u8,
}

/// Temporary struct for deserialization purposes
#[derive(Debug, Serialize, Deserialize)]
pub struct TempItem {
    pub id: String,
    pub kind: ItemType,
    pub stack: u8,
}

/// Type of armor piece
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArmorType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
}

/// Type of item
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
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

impl Default for BlockData {
    fn default() -> Self {
        Self {
            drops: vec![(1, 0)],
            break_time: 1.,
            uvs: [0.25, 0.5, 0.0, 1.0],
        }
    }
}

#[derive(Resource, Debug)]
pub struct ItemBlockRegistry {
    pub blocks: HashMap<BlockId, BlockData>,
    pub items: HashMap<ItemId, ItemData>,
    pub block_to_id: HashMap<String, BlockId>,
    pub item_to_id: HashMap<String, ItemId>,
}

/// Loads all blocks and items into the registry from their `.ron` files
pub fn load_blocks_items(mut registry: ResMut<ItemBlockRegistry>) {
    println!("Begin items & blocks loading...");

    // Create String -> Id maps
    let mut block_to_id: HashMap<String, BlockId> = HashMap::new();
    let mut item_to_id: HashMap<String, ItemId> = HashMap::new();

    // First, load all items
    for p in fs::read_dir("../data/items").unwrap().flatten() {
        if let Ok(contents) = fs::read_to_string(p.path()) {
            if let Ok(item) = from_str::<TempItem>(&contents) {
                // Gets numeric id of item after registration
                let nb = registry.items.len() as ItemId;
                // Maps string id to numeric id for blocks
                item_to_id.insert(item.id, nb);

                // Insert item into registry
                registry.items.insert(
                    nb,
                    ItemData {
                        kind: item.kind,
                        stack: item.stack,
                    },
                );
            }
        }
    }

    // Then, load all blocks
    for p in fs::read_dir("../data/blocks").unwrap().flatten() {
        if let Ok(contents) = fs::read_to_string(p.path()) {
            if let Ok(block) = from_str::<TempBlock>(&contents) {
                // Gets numeric id of block after registration
                let nb = registry.blocks.len() as BlockId;
                // Maps string id to numeric id for items
                block_to_id.insert(block.id, nb);

                // Inserts block into registry
                registry.blocks.insert(
                    nb,
                    BlockData {
                        drops: {
                            let mut d: Vec<(u16, ItemId)> = Vec::new();
                            for drop in block.drops {
                                // Gets numeric id of item drop
                                d.push((drop.0, *item_to_id.get(&drop.1).unwrap()));
                            }
                            d
                        },
                        break_time: block.break_time,
                        uvs: block.uvs,
                    },
                );
            }
        }
    }

    // Finally, edit items with numeric ids of blocks
    for (txt, id) in item_to_id.iter() {
        if let Some(item) = registry.items.get_mut(id) {
            if item.kind == ItemType::Block(0) {
                item.kind = ItemType::Block(*block_to_id.get(txt).unwrap());
            }
        }
    }

    registry.block_to_id = block_to_id;
    registry.item_to_id = item_to_id;

    println!("--------------------------------------------");
    println!(
        "Final items :  {:?}\n\n{:?}\n\n{:?}\n\n{:?}",
        registry.blocks, registry.items, registry.block_to_id, registry.item_to_id
    );
    println!("--------------------------------------------");
}

pub enum TestType {
    Grass,
    Dirt,
    Stone,
    Bedrock,
}

pub enum BlockType {
    Dirt,
    Grass,
    Stone,
    Bedrock,
}

pub trait BlockEnum {
    fn get_id(self) -> String;
}

impl BlockEnum for BlockType {
    fn get_id(self) -> String {
        match self {
            BlockType::Grass => "grass".into(),
            BlockType::Bedrock => "bedrock".into(),
            BlockType::Dirt => "dirt".into(),
            BlockType::Stone => "stone".into(),
        }
    }
}
