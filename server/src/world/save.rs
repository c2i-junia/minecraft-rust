use crate::world::data::*;
use bevy::prelude::*;
use ron::ser::PrettyConfig;
use shared::world::get_game_folder;
use shared::world::ServerWorldMap;
use shared::world::WorldSeed;
use shared::GameFolderPath;
use std::{fs::File, io::Write, path::Path};

#[derive(Event)]
pub struct SaveRequestEvent;

use crate::world::data::SAVE_PATH;

// System to save the world when "L" is pressed
pub fn save_world_system(
    world_map: ResMut<ServerWorldMap>,
    world_seed: Res<WorldSeed>, // Add `WorldSeed` as a resource here
    // inventory: Res<Inventory>,
    // player_query: Query<&Transform>,
    game_folder_path: Res<GameFolderPath>,
    mut event: EventReader<SaveRequestEvent>,
) {
    let mut save_requested = false;

    // Reads all events to prevent them from being queued forever and repeatedly request a save
    for _ in event.read() {
        save_requested = true;
    }

    // If a save was requested by the user
    if save_requested {
        // let transform = match player_query.iter().next() {
        //     Some(transform) => transform,
        //     None => {
        //         error!("No player transform found!");
        //         return;
        //     }
        // };

        let data = Save {
            map: world_map.map.clone(),
            // player_pos: transform.translation,
            // inventory: inventory.inner.clone(),
            // id_to_item: {
            //     // Same for ItemId -> String
            //     let mut rimap: HashMap<RegistryId, String> = HashMap::new();
            //     for (key, value) in r_items.iter_names() {
            //         rimap.insert(*value, key.clone());
            //     }
            //     rimap
            // },
        };

        // Save the world and the seed into their respective files
        if let Err(e) = save_world_map(
            &data,
            &format!(
                "{}{}_save.ron",
                get_game_folder(Some(&game_folder_path))
                    .join(SAVE_PATH)
                    .display(),
                world_map.name
            ),
        ) {
            error!("Failed to save world: {}", e);
        } else {
            info!("World saved successfully! Name: {}", world_map.name);
        }

        if let Err(e) = save_world_seed(
            &world_seed,
            &format!(
                "{}{}_seed.ron",
                get_game_folder(Some(&game_folder_path))
                    .join(SAVE_PATH)
                    .display(),
                world_map.name
            ),
        ) {
            error!("Failed to save world seed: {}", e);
        } else {
            info!("World seed saved successfully!");
        }
    }
}

pub fn save_world_map(save: &Save, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Use RON to serialize `ServerWorldMap`
    let pretty_config = PrettyConfig::new()
        .with_depth_limit(3)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    let serialized = ron::ser::to_string_pretty(save, pretty_config)?; // Serialize with a readable format
    let path = Path::new(file_path);
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    info!("ServerWorldMap saved to {}", file_path);
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
    let serialized = ron::ser::to_string_pretty(world_seed, pretty_config)?; // Serialize with a readable format
    let path = Path::new(file_path);
    let mut file = File::create(path)?;
    file.write_all(serialized.as_bytes())?;
    info!("WorldSeed saved to {}", file_path);
    Ok(())
}
