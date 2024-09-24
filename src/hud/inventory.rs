use crate::player::Player;
use bevy::prelude::*;

#[derive(Component)]
pub struct InventoryText;

pub fn inventory_text_update_system(
    player: Query<&Player>,
    mut query: Query<&mut Text, With<InventoryText>>,
) {
    for mut text in query.iter_mut() {
        let player = player.single();
        text.sections[0].value = format!("Inventory: {:?}", player.inventory);
    }
}