use std::fs;
use std::path::Path;

use crate::world::data::{WorldMap, WorldSeed};
use ron::de::from_str;

pub fn load_world_map(file_path: &str) -> Result<WorldMap, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let contents = fs::read_to_string(path)?;
    let world_map: WorldMap = from_str(&contents)?; // Désérialisation avec RON
    println!("WorldMap loaded from {}", file_path);
    Ok(world_map)
}

pub fn load_world_seed(file_path: &str) -> Result<WorldSeed, Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    let contents = fs::read_to_string(path)?;
    let world_seed: WorldSeed = from_str(&contents)?; // Désérialisation avec RON
    Ok(world_seed)
}
