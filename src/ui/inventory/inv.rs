use super::{add_item_floating_stack, remove_item_floating_stack};
use crate::player::inventory::{add_item_to_stack, remove_item_from_stack};
use crate::ui::inventory::items::Item;
use crate::{constants::MAX_ITEM_STACK, player::Player};
use bevy::prelude::*;

/// Marker for Inventory root
#[derive(Component)]
pub struct InventoryRoot;

/// Main inventory dialog
#[derive(Component)]
pub struct InventoryDialog;

#[derive(Component)]
pub struct InventoryCell {
    pub id: u32,
}

/// The current selected stack, not considered in the player's inventory
#[derive(Component)]
pub struct FloatingStack {
    pub items: Option<Item>,
}

pub fn inventory_cell_interaction_system(
    mut cursor_query: Query<(&Interaction, &mut BorderColor, &InventoryCell), With<InventoryCell>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut player_query: Query<&mut Player>,
    mut floating_stack: Query<&mut FloatingStack>,
) {
    let mut player = player_query.single_mut();
    let mut floating_stack = floating_stack.single_mut();
    for (interaction, mut border_color, cell) in &mut cursor_query {
        if *interaction == Interaction::None {
            border_color.0 = Color::BLACK;
            continue;
        }
        // Means we have an interaction with the cell, but which type of interaction ?

        let stack = player.inventory.get(&cell.id);
        let floating_items = floating_stack.items;

        // Using variables to avoid E0502 errors -_-
        let stack_exists = stack.is_some();
        let floating_exists = floating_items.is_some();

        // In case LMB pressed :
        if mouse_input.just_pressed(MouseButton::Left) {
            // Transfer items from inventory cell to floating stack

            if stack_exists
                && floating_exists
                && stack.unwrap().id == floating_items.unwrap().id
                && stack.unwrap().nb < MAX_ITEM_STACK
            {
                let stack = *stack.unwrap();
                let floating_items = floating_items.unwrap();
                add_item_to_stack(
                    &mut player,
                    floating_items.id,
                    cell.id,
                    remove_item_floating_stack(&mut floating_stack, MAX_ITEM_STACK - stack.nb),
                );
            } else {
                if stack_exists {
                    let stack = stack.unwrap();
                    floating_stack.items = Some(*stack);
                    // If no exchange is made with floating stack, clear cell
                    if !floating_exists {
                        player.inventory.remove(&cell.id);
                    }
                }

                // Transfer items from floating stack to inventory cell
                if floating_exists {
                    let floating_items = floating_items.unwrap();
                    player.inventory.insert(cell.id, floating_items);
                    // If no exchange is made with cell, clear floating stack
                    if !stack_exists {
                        floating_stack.items = None;
                    }
                }
            }
        }
        // Welcome to nesting hell
        else if mouse_input.just_pressed(MouseButton::Right) {
            // If holding stack : remove 1 item from floating stack
            if floating_exists {
                let floating_items = floating_items.unwrap();

                if stack_exists {
                    let stack = stack.unwrap();

                    if floating_items.id == stack.id && floating_items.nb > 0 {
                        // Get added nb of items into inventory -> removes them from floating stack

                        remove_item_floating_stack(
                            &mut floating_stack,
                            add_item_to_stack(&mut player, floating_items.id, cell.id, 1),
                        );
                    }
                } else if floating_items.nb > 0 {
                    // Get added nb of items into inventory -> removes them from floating stack
                    remove_item_floating_stack(
                        &mut floating_stack,
                        add_item_to_stack(&mut player, floating_items.id, cell.id, 1),
                    );
                }
            }
            // Else if hovering a stack : cut hovered stack in half (rounded up), and push it to floating stack
            else if stack_exists {
                let stack = *stack.unwrap();
                let nb = (stack.nb + 1) / 2;
                // Get removed nb of items removed from inventory -> adds them into the floating stack
                add_item_floating_stack(
                    &mut floating_stack,
                    remove_item_from_stack(&mut player, stack.id, cell.id, nb),
                    stack.id,
                );
            }
        } else {
            border_color.0 = Color::WHITE;
        }
    }
}
