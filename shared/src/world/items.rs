use std::fs;
use std::collections::HashMap;

use bevy::{
    prelude::{ResMut, Resource},
    scene::ron::from_str,
};
use serde::{Deserialize, Serialize};

pub type BlockId = u32;
pub type ItemId = u32;

#[derive(Debug, Clone)]
pub struct BlockData {
    pub drops: Vec<(u16, ItemId)>,
    pub break_time: f32,
    pub uvs: [f32; 4],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TempBlock {
    pub id: String,
    pub drops: Vec<(u16, String)>,
    pub break_time: f32,
    pub uvs: [f32; 4],
}

#[derive(Debug)]
pub struct ItemData {
    pub kind: ItemType,
    pub stack: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TempItem {
    pub id: String,
    pub kind: ItemType,
    pub stack: u8,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArmorType {
    Helmet,
    Chestplate,
    Leggings,
    Boots,
}

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
    println!("Test");

    // let mut block_to_id: HashMap<String, BlockId> = HashMap::new();
    // let mut item_to_id: HashMap<String, ItemId> = HashMap::new();

    // First, load all items
    for p in fs::read_dir("../data/items").unwrap() {
        if let Ok(p) = p {
            if let Ok(contents) = fs::read_to_string(p.path()) {
                if let Ok(item) = from_str::<TempItem>(&contents) {

                    // Gets numeric id of item after registration
                    let nb = registry.items.len() as ItemId;
                    // Maps string id to numeric id for blocks
                    registry.item_to_id.insert(item.id, nb);

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
    }

    // Clone item -> id map for borrow purposes
    let clone_item = registry.item_to_id.clone();

    // Then, load all blocks
    for p in fs::read_dir("../data/blocks").unwrap() {
        if let Ok(p) = p {
            if let Ok(contents) = fs::read_to_string(p.path()) {
                if let Ok(block) = from_str::<TempBlock>(&contents) {

                    // Gets numeric id of block after registration
                    let nb = registry.blocks.len() as BlockId;
                    // Maps string id to numeric id for items
                    registry.block_to_id.insert(block.id, nb);

                    // Inserts block into registry
                    registry.blocks.insert(
                        nb,
                        BlockData {
                            drops: {
                                let mut d: Vec<(u16, ItemId)> = Vec::new();
                                for drop in block.drops {
                                    // Gets numeric id of item drop
                                    d.push((drop.0, *clone_item.get(&drop.1).unwrap()));
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
    }

    // Clone block -> id map for borrow purposes
    let clone_block = registry.block_to_id.clone();

    // Finally, edit items with numeric ids of blocks
    for (txt, id) in clone_item.iter() {
        if let Some(item) = registry.items.get_mut(id) {
            if item.kind == ItemType::Block(0) {
                item.kind = ItemType::Block(*clone_block.get(txt).unwrap());
            }
        }
    }

    println!("--------------------------------------------");
    println!("Final items :  {:?}\n\n{:?}\n\n{:?}\n\n{:?}", registry.blocks, registry.items, registry.block_to_id, registry.item_to_id);
    println!("--------------------------------------------");
}
