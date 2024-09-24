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
        // Check if inventory is empty
        if player.inventory.is_empty() {
            text.sections[0].value = "Inventory: Empty".to_string();
            return;
        }
        // Update inventory text
        text.sections[0].value = format!("Inventory: {:?}", player.inventory);
    }
}