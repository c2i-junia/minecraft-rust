use crate::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct CoordsText;

pub fn coords_text_update_system(
    player: Query<&Transform, With<Player>>,
    mut query: Query<&mut Text, With<CoordsText>>,
) {
    for mut text in query.iter_mut() {
        let coords = player.single();
        text.sections[0].value = format!(
            "X/Y/Z = {:.2}/{:.2}/{:.2}",
            coords.translation.x, coords.translation.y, coords.translation.z
        );
    }
}
