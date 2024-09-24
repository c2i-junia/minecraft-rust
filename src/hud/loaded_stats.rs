use crate::world::WorldMap;
use bevy::prelude::*;

#[derive(Component)]
pub struct BlocksNumberText;

#[derive(Component)]
pub struct ChunksNumberText;

pub fn total_blocks_text_update_system(
    mut query_blocks: Query<&mut Text, With<BlocksNumberText>>,
    mut query_chunks: Query<&mut Text, (With<ChunksNumberText>, Without<BlocksNumberText>)>,
    world_map: Res<WorldMap>,
) {
    for mut text in query_blocks.iter_mut() {
        text.sections[0].value = format!("Loaded blocks: {}", world_map.total_blocks_count);
    }
    for mut text in query_chunks.iter_mut() {
        text.sections[0].value = format!("Loaded chunks: {}", world_map.total_chunks_count);
    }
}
