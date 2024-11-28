use crate::player::CurrentPlayerMarker;
use bevy::prelude::*;
use shared::world::block_to_chunk_coord;

#[derive(Component)]
pub struct CoordsText;

pub fn coords_text_update_system(
    player: Query<&Transform, With<CurrentPlayerMarker>>,
    mut query: Query<&mut Text, With<CoordsText>>,
) {
    for mut text in query.iter_mut() {
        let coords = player.single();
        let player_chunk = IVec3::new(
            block_to_chunk_coord(coords.translation.x as i32),
            block_to_chunk_coord(coords.translation.y as i32),
            block_to_chunk_coord(coords.translation.z as i32),
        );
        text.sections[0].value = format!(
            "X/Y/Z = {:.2}/{:.2}/{:.2}\nChunk pos : {:?}",
            coords.translation.x, coords.translation.y, coords.translation.z, player_chunk
        );
    }
}
