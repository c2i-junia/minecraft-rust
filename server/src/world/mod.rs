pub mod broadcast;
mod data;
pub mod generation;
pub mod load_from_file;
pub mod save;
mod utils;

use bevy::prelude::Event;
use bevy::prelude::EventReader;
use bevy::prelude::IVec3;
use bevy::prelude::ResMut;
use bevy::prelude::*;
pub use broadcast::*;
use shared::world::BlockData;
use shared::world::ServerWorldMap;

#[derive(Event, Debug)]
pub struct BlockInteractionEvent {
    pub position: IVec3,
    pub block_type: Option<BlockData>, // None = suppression, Some = ajout
}

pub fn handle_block_interactions(
    mut world_map: ResMut<ServerWorldMap>,
    mut events: EventReader<BlockInteractionEvent>,
) {
    for event in events.read() {
        match &event.block_type {
            Some(block) => {
                // Ajouter un bloc
                world_map.set_block(&event.position, *block);
                debug!("Block added at {:?}: {:?}", event.position, block);
            }
            None => {
                // Supprimer un bloc
                world_map.remove_block_by_coordinates(&event.position);
                info!("Block removed at {:?}", event.position);
            }
        }
    }
}
