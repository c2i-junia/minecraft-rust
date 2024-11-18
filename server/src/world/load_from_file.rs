use bevy::prelude::*;
use ron::de::from_str;
use shared::world::data::{ServerWorldMap, WorldSeed};
use shared::world::get_game_folder;
use std::fs;
use std::path::Path;

use crate::world::data::Save;
use crate::world::data::SAVE_PATH;

pub fn load_world_map(
    file_name: &str,
    // player: &mut Player,
    // player_pos: &mut Vec3,
    // r_items: &Registry<ItemData>,
) -> Result<ServerWorldMap, Box<dyn std::error::Error>> {
    let file_path: String = format!(
        "{}{}_save.ron",
        get_game_folder().join(SAVE_PATH).display(),
        file_name
    );
    let path: &Path = Path::new(&file_path);

    if !path.exists() {
        info!(
            "World map file not found: {}. Returning default world.",
            file_path
        );
        let mut default_map = ServerWorldMap::default();
        default_map.name = file_name.to_string(); // Toujours mettre le nom du monde
        return Ok(default_map);
    }

    let contents: String = fs::read_to_string(path)?;
    let save: Save = from_str::<Save>(&contents)?; // Deserialization using RON

    let world_map: ServerWorldMap = ServerWorldMap {
        name: file_name.into(),
        map: save.map,
        ..Default::default()
    };

    Ok(world_map)
}

pub fn load_world_seed(file_name: &str) -> Result<WorldSeed, Box<dyn std::error::Error>> {
    let file_path: String = format!(
        "{}{}_seed.ron",
        get_game_folder().join(SAVE_PATH).display(),
        file_name
    );
    let path: &Path = Path::new(&file_path);

    if !path.exists() {
        info!(
            "World seed file not found: {}. Generating a random seed.",
            file_path
        );
        return Ok(WorldSeed(rand::random::<u32>()));
    }

    let contents: String = fs::read_to_string(path)?;
    let world_seed: WorldSeed = from_str(&contents)?; // Deserialization using RON
    Ok(world_seed)
}
