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
use bevy::math::Vec3;
use ron::de::from_str;

pub fn load_world_map(
    file_name: &str,
    player: &mut Player,
    player_pos: &mut Vec3,
) -> Result<WorldMap, Box<dyn std::error::Error>> {
    let file_path = format!("{}{}_save.ron", SAVE_PATH, file_name);
    let path = Path::new(&file_path);
    let contents = fs::read_to_string(path)?;
    let save = from_str::<Save>(&contents)?; // Désérialisation avec RON

    let world_map = WorldMap {
        map: save.map,
        ..Default::default()
    };

    player.inventory = save.inventory;

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
