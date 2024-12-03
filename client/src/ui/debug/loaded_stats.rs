use crate::world::time::ClientTime;
use crate::world::ClientWorldMap;
use bevy::prelude::*;

#[derive(Component)]
pub struct BlocksNumberText;

#[derive(Component)]
pub struct TimeText;

#[derive(Component)]
pub struct ChunksNumberText;

pub fn total_blocks_text_update_system(
    mut query_blocks: Query<&mut Text, With<BlocksNumberText>>,
    mut query_chunks: Query<&mut Text, (With<ChunksNumberText>, Without<BlocksNumberText>)>,
    world_map: Res<ClientWorldMap>,
) {
    for mut text in query_blocks.iter_mut() {
        text.sections[0].value = format!("Loaded blocks: {}", world_map.total_blocks_count);
    }
    for mut text in query_chunks.iter_mut() {
        text.sections[0].value = format!("Loaded chunks: {}", world_map.map.len());
    }
}

pub fn time_text_update_system(
    mut query: Query<&mut Text, With<TimeText>>,
    time_resource: Res<ClientTime>,
) {
    for mut text in query.iter_mut() {
        text.sections[0].value = format!("Time: {}", time_resource.0);
    }
}
