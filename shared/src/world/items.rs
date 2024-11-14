use crate::world::get_game_folder;
use std::collections::hash_map::Iter;
use std::fs;
use std::{collections::HashMap, fmt::Debug};

use bevy::{
    prelude::{ResMut, Resource},
    scene::ron::from_str,
};
use serde::{Deserialize, Serialize};

pub type BlockId = u32;
pub type ItemId = u32;

/// Data associated with a given `BlockId`
#[derive(Debug, Clone)]
pub struct BlockData {
    pub drops: Vec<(u16, RegistryId)>,
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
#[derive(Debug, Clone, Copy)]
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
    Block(RegistryId),
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

impl Default for TempBlock {
    fn default() -> Self {
        Self {
            id: "dirt".into(),
            drops: vec![(1, "dirt".into())],
            break_time: 1.,
            uvs: [0.25, 0.5, 0.0, 1.0],
        }
    }
}

pub type RegistryId = u32;

/// Structure for registering static data pertaining to a certain object in gameplay
///
/// For example : items, blocks, mobs...
#[derive(Resource, Debug, Clone)]
pub struct Registry<T> {
    inner: HashMap<RegistryId, T>,
    name_to_id: HashMap<String, RegistryId>,
}

impl<T> Default for Registry<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Registry<T> {
    /// New empty registry
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
            name_to_id: HashMap::new(),
        }
    }

    /// Register a new object in the registry
    ///
    /// If an item with the same `name` is already present, it will be overwritten
    pub fn register(&mut self, name: String, block: T) -> RegistryId {
        let len = if let Some(n) = self.name_to_id.get(&name) {
            *n as RegistryId
        } else {
            self.inner.len() as RegistryId
        };

        self.inner.insert(len, block);
        self.name_to_id.insert(name, len);
        len
    }

    /// Replaces the item with the same `id`, **without changing its `name`**
    pub fn edit(&mut self, id: RegistryId, item: T) -> Option<T> {
        self.inner.insert(id, item)
    }

    /// Tries to remove an object from the registry by its `name`, then returns it if it exists
    pub fn remove(&mut self, name: &str) -> Option<T> {
        if let Some(id) = self.name_to_id.remove(name) {
            self.inner.remove(&id)
        } else {
            None
        }
    }

    /// Tries to get a reference to an object from its `id`
    pub fn get(&self, block_id: &RegistryId) -> Option<&T> {
        self.inner.get(block_id)
    }

    /// Tries to get a reference to a mutable object from its `id`
    pub fn get_mut(&mut self, block_id: &RegistryId) -> Option<&mut T> {
        self.inner.get_mut(block_id)
    }

    /// Tries to get the `id` of an object from its `name`
    pub fn get_id(&self, block_name: &str) -> Option<&RegistryId> {
        self.name_to_id.get(block_name)
    }

    /// Tries to get a reference to an object from its `name`
    pub fn get_by_name(&self, block_name: &str) -> Option<&T> {
        if let Some(id) = self.name_to_id.get(block_name) {
            return self.inner.get(id);
        }
        None
    }

    /// Iterates through the `(id, object)` pairs of the registry
    pub fn iter(&self) -> Iter<RegistryId, T> {
        self.inner.iter()
    }

    /// Iterates through the `(name, id)` pairs of the registry
    pub fn iter_names(&self) -> Iter<String, RegistryId> {
        self.name_to_id.iter()
    }
}

// Loads all blocks and items into the registry from their `.ron` files
pub fn load_blocks_items(
    mut item_r: ResMut<Registry<ItemData>>,
    mut blocks_r: ResMut<Registry<BlockData>>,
) {
    println!("Begin items & blocks loading...");

    // First, load all items
    for p in fs::read_dir(get_game_folder().join("data/items"))
        .unwrap()
        .flatten()
    {
        if let Ok(contents) = fs::read_to_string(p.path()) {
            if let Ok(item) = from_str::<TempItem>(&contents) {
                // Insert item into registry
                item_r.register(
                    item.id,
                    ItemData {
                        kind: item.kind,
                        stack: item.stack,
                    },
                );
            }
        }
    }

    // Then, load all blocks
    for p in fs::read_dir(get_game_folder().join("data/blocks"))
        .unwrap()
        .flatten()
    {
        if let Ok(contents) = fs::read_to_string(p.path()) {
            if let Ok(block) = from_str::<TempBlock>(&contents) {
                let mut d: Vec<(u16, RegistryId)> = Vec::new();
                for drop in block.drops {
                    // Gets numeric id of item drop
                    d.push((drop.0, *item_r.get_id(&drop.1).unwrap()));
                }

                // Inserts block into registry
                blocks_r.register(
                    block.id,
                    BlockData {
                        drops: d,
                        break_time: block.break_time,
                        uvs: block.uvs,
                    },
                );
            }
        }
    }

    // Finally, edit items with numeric ids of blocks
    for (txt, id) in item_r.clone().iter_names() {
        let mut item: ItemData = *item_r.get(id).unwrap();
        if item.kind == ItemType::Block(0) {
            item.kind = ItemType::Block(*blocks_r.get_id(txt).unwrap());
        }
        item_r.edit(*id, item);
    }

    println!("--------------------------------------------");
    println!("Final items :  {:?}\n\n{:?}", blocks_r, item_r);
    println!("--------------------------------------------");
}

pub enum BlockType {
    Dirt,
    Grass,
    Stone,
    Bedrock,
}

impl BlockType {
    pub fn get_name(self) -> String {
        match self {
            BlockType::Grass => "grass".into(),
            BlockType::Bedrock => "bedrock".into(),
            BlockType::Dirt => "dirt".into(),
            BlockType::Stone => "stone".into(),
        }
    }
}
