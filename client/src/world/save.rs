use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::{fs, io};


use crate::constants::SAVE_PATH;
use crate::world::data::*;
use bevy::prelude::*;

use ron::ser::PrettyConfig;

#[derive(Event)]
pub struct SaveRequestEvent;

// Système pour sauvegarder le monde lorsque "L" est pressé
pub fn save_world_system(
    world_map: Res<WorldMap>,
    world_seed: Res<WorldSeed>, // Ajoute `WorldSeed` comme ressource ici
    mut event: EventReader<SaveRequestEvent>
) {
    let mut save_requested = false;

    // Reads all events to prevent them from being queued forever and repeatedly request a save
    for _ in event.read() {
        save_requested = true;
    }

    // If a save was requested by the user
    if save_requested {

        // Sauvegarde le monde et la graine dans leurs fichiers respectifs
        if let Err(e) = save_world_map(&world_map, &format!("{}{}_save.ron", SAVE_PATH, world_map.name)) {
            eprintln!("Failed to save world: {}", e);
        } else {
            println!("World saved successfully!");
        }

        if let Err(e) = save_world_seed(&world_seed, &format!("{}{}_seed.ron", SAVE_PATH, world_map.name)) {
            eprintln!("Failed to save world seed: {}", e);
        } else {
            println!("World seed saved successfully!");
        }
    }
}

pub fn save_world_map(
    world_map: &WorldMap,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    // Utilise RON pour sérialiser `WorldMap`
    let pretty_config = PrettyConfig::new()
        .with_depth_limit(3)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    let serialized = ron::ser::to_string_pretty(world_map, pretty_config)?; // Sérialise avec un format lisible
    let path = Path::new(file_path);
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    println!("WorldMap saved to {}", file_path);
    Ok(())
}

pub fn save_world_seed(
    world_seed: &WorldSeed,
    file_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let pretty_config = PrettyConfig::new()
        .with_depth_limit(3)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    let serialized = ron::ser::to_string_pretty(world_seed, pretty_config)?; // Sérialise avec un format lisible
    let path = Path::new(file_path);
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    println!("WorldSeed saved to {}", file_path);
    Ok(())
}

pub fn delete_save_files(world_name: &str) -> Result<(), io::Error> {
    // Supprime `world_save.ron`
    match fs::remove_file(&format!("{}{}_save.ron", SAVE_PATH, world_name)) {
        Ok(_) => println!("Successfully deleted world_save.ron"),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            println!("world_save.ron not found, skipping.")
        }
        Err(e) => println!("Failed to delete world_save.ron: {}", e),
    }

    // Supprime `world_seed.ron`
    match fs::remove_file(&format!("{}_seed.ron", world_name)) {
        Ok(_) => println!("Successfully deleted world_seed.ron"),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            println!("world_seed.ron not found, skipping.")
        }
        Err(e) => println!("Failed to delete world_seed.ron: {}", e),
    }

    Ok(())
}