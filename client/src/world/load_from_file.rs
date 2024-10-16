use std::fs;
use std::path::Path;

use crate::{
    constants::SAVE_PATH,
    player::Player,
    world::{
        data::{WorldMap, WorldSeed},
        Save,
    },
};
use bevy::{math::Vec3, utils::hashbrown::HashMap};
use ron::de::from_str;
use shared::world::{BlockData, Item, ItemData, Registry, RegistryId};

pub fn load_world_map(
    file_name: &str,
    player: &mut Player,
    player_pos: &mut Vec3,
    r_items: &Registry<ItemData>,
    r_blocks: &Registry<BlockData>,
) -> Result<WorldMap, Box<dyn std::error::Error>> {
    let file_path = format!("{}{}_save.ron", SAVE_PATH, file_name);
    let path = Path::new(&file_path);
    let contents = fs::read_to_string(path)?;
    let mut save = from_str::<Save>(&contents)?; // Désérialisation avec RON

    // Build map : old ItemId -> new ItemId, in case the blocks aren't the same
    let mut items_changed = false;
    let mut items_id_map: HashMap<RegistryId, RegistryId> = HashMap::new();
    for (old_id, name) in save.id_to_item.iter() {
        if let Some(new_id) = r_items.get_id(name) {
            if new_id != old_id {
                items_changed = true;
            }
            items_id_map.insert(*old_id, *new_id);
        }
    }

    // Same for blocks
    let mut blocks_changed = false;
    let mut blocks_id_map: HashMap<RegistryId, RegistryId> = HashMap::new();
    for (old_id, name) in save.id_to_block.iter() {
        if let Some(new_id) = r_blocks.get_id(name) {
            if new_id != old_id {
                blocks_changed = true;
            }
            blocks_id_map.insert(*old_id, *new_id);
        }
    }

    let world_map = WorldMap {
        name: file_name.into(),
        map: {
            if blocks_changed {
                for (_, chunk) in save.map.iter_mut() {
                    for (pos, block) in chunk.map.clone().iter() {
                        if let Some(block_id) = blocks_id_map.get(block) {
                            chunk.map.insert(*pos, *block_id);
                        } else {
                            chunk.map.remove(pos);
                        }
                    }
                }
            }
            save.map
        },
        ..Default::default()
    };

    player.inventory = if items_changed {
        let mut inv = save.inventory.clone();
        for (id, item) in save.inventory.iter() {
            if let Some(item_id) = items_id_map.get(&item.id) {
                inv.insert(
                    *id,
                    Item {
                        id: *item_id,
                        nb: item.nb,
                    },
                );
            } else {
                inv.remove(id);
            }
        }
        inv
    } else {
        save.inventory
    };

    *player_pos = save.player_pos;

    Ok(world_map)
}

pub fn load_world_seed(file_name: &str) -> Result<WorldSeed, Box<dyn std::error::Error>> {
    let file_path = format!("{}{}_seed.ron", SAVE_PATH, file_name);
    let path = Path::new(&file_path);
    let contents = fs::read_to_string(path)?;
    let world_seed: WorldSeed = from_str(&contents)?; // Désérialisation avec RON
    Ok(world_seed)
}
