use crate::constants::{MAX_HOTBAR_SLOTS, MAX_ITEM_STACK};
use crate::hotbar::Hotbar;
use crate::input::keyboard::{get_action_keys, GameAction};
use crate::inventory::{add_item_to_stack, remove_item_from_stack};
use crate::player::Player;
use crate::ui::{FloatingStack, InventoryCell, InventoryRoot};
use crate::world::MaterialResource;
use bevy::color::Color;
use bevy::hierarchy::Children;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::input::ButtonInput;
use bevy::prelude::{
    EventReader, KeyCode, MouseButton, Query, Res, ResMut, Style, Text, UiImage, Val, Visibility,
    Window, With,
};
use bevy::render::texture::TRANSPARENT_IMAGE_HANDLE;
use bevy::ui::{BorderColor, Interaction};
use bevy::window::PrimaryWindow;

use super::{add_item_floating_stack, remove_item_floating_stack};

pub fn render_inventory_hotbar(
    queries: (
        Query<&mut Player>,
        Query<&mut Text>,
        Query<&mut UiImage>,
        Query<(&mut Style, &mut FloatingStack, &Children), With<FloatingStack>>,
        Query<(&Interaction, &mut BorderColor, &InventoryCell, &Children), With<InventoryCell>>,
        Query<&mut Visibility, With<InventoryRoot>>,
        Query<&Window, With<PrimaryWindow>>,
        Query<&mut Hotbar>,
    ),
    resources: (
        ResMut<ButtonInput<KeyCode>>,
        Res<ButtonInput<MouseButton>>,
        Res<MaterialResource>,
    ),
    mut scroll: EventReader<MouseWheel>,
) {
    let (
        mut player_query,
        mut text_query,
        mut image_query,
        mut floating_stack_query,
        mut cursor_query,
        mut visibility_query,
        window_query,
        mut hotbar_query,
    ) = queries;

    let (mut keyboard_input, mouse_input, material_resource) = resources;

    let mut vis = visibility_query.single_mut();
    let keys = get_action_keys(GameAction::ToggleInventory);
    for key in keys {
        if keyboard_input.just_pressed(key) {
            *vis = match *vis {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }

    let mut player = player_query.single_mut();

    let (mut style, mut floating_stack, children) = floating_stack_query.single_mut();
    let mut txt = text_query.get_mut(children[0]).unwrap();
    let mut img = image_query.get_mut(children[1]).unwrap();

    // Change selected stack via scrolling
    let mut stack_scrolling = hotbar_query.single().selected as i32;
    for sc in scroll.read() {
        match sc.unit {
            MouseScrollUnit::Line => {
                stack_scrolling -= sc.y as i32;
            }
            MouseScrollUnit::Pixel => {
                stack_scrolling -= sc.y as i32 / 20;
            }
        }
    }

    // Add scrolling
    hotbar_query.single_mut().selected = stack_scrolling.rem_euclid(MAX_HOTBAR_SLOTS as i32) as u32;

    // Set content
    if floating_stack.items.is_none() {
        txt.sections[0].value = "".to_string();
        img.texture = TRANSPARENT_IMAGE_HANDLE;
    } else {
        let fstack = floating_stack.items.unwrap();
        txt.sections[0].value = format!("{:?}", fstack.nb);
        img.texture = material_resource
            .item_textures
            .get(&fstack.id)
            .unwrap()
            .clone();
    }

    if let Some(c_pos) = window_query.single().cursor_position() {
        style.top = Val::Px(c_pos.y);
        style.left = Val::Px(c_pos.x);
    }

    for (interaction, mut border_color, cell, children) in cursor_query.iter_mut() {
        // Don't update hidden cells, waste of resources
        if cell.id >= MAX_HOTBAR_SLOTS && *vis != Visibility::Visible {
            return;
        }

        let stack = player.inventory.get(&cell.id);
        let mut txt = text_query.get_mut(children[0]).unwrap();
        let mut img = image_query.get_mut(children[1]).unwrap();

        // Update visual content
        if stack.is_none() {
            txt.sections[0].value = "".to_string();
            img.texture = TRANSPARENT_IMAGE_HANDLE;
        } else {
            let stack = stack.unwrap();
            txt.sections[0].value = format!("{:?}", stack.nb);
            img.texture = material_resource
                .item_textures
                .get(&stack.id)
                .unwrap()
                .clone();
        }

        // Show selected stack in hotbar
        if *vis != Visibility::Visible && hotbar_query.single().selected == cell.id {
            border_color.0 = Color::WHITE;
            continue;
        }

        // If no interaction (or the inventory is closed for hotbar), the border is the default one
        if *interaction == Interaction::None || *vis != Visibility::Visible {
            border_color.0 = Color::srgb(0.3, 0.3, 0.3);
            continue;
        }
        // Means we have an interaction with the cell, but which type of interaction ?

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
            // If floating stack exists : remove 1 item from floating stack
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
                    remove_item_from_stack(&mut player, cell.id, nb),
                    stack.id,
                );
            }
        } else {
            border_color.0 = Color::WHITE;
        }
    }

    keyboard_input.reset_all();
}
