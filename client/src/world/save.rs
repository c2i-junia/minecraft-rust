use std::fs::File;
use std::io::Write;
use std::path::Path;

use crate::input::keyboard::*;
use crate::world::data::*;
use bevy::prelude::*;

use ron::ser::PrettyConfig;

// Système pour sauvegarder le monde lorsque "L" est pressé
pub fn save_world_system(
    keyboard_input: Res<ButtonInput<KeyCode>>, // Corrigé pour `Input<KeyCode>`
    world_map: Res<WorldMap>,
    world_seed: Res<WorldSeed>, // Ajoute `WorldSeed` comme ressource ici
) {
    // Vérifie si l'action `GameAction::SaveWorld` est pressée (associe cela à la touche `L`)
    if is_action_pressed(GameAction::SaveWorld, &keyboard_input) {
        // Sauvegarde le monde et la graine dans leurs fichiers respectifs
        if let Err(e) = save_world_map(&world_map, "world_save.ron") {
            eprintln!("Failed to save world: {}", e);
        } else {
            println!("World saved successfully!");
        }

        if let Err(e) = save_world_seed(&world_seed, "world_seed.ron") {
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
