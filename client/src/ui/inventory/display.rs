use super::{add_item_floating_stack, remove_item_floating_stack};
use crate::constants::MAX_HOTBAR_SLOTS;
use crate::input::data::GameAction;
use crate::input::keyboard::is_action_just_pressed;
use crate::player::inventory::Inventory;
use crate::ui::hotbar::Hotbar;
use crate::ui::{FloatingStack, InventoryCell, InventoryRoot};
use crate::world::MaterialResource;
use crate::KeyMap;
use bevy::color::Color;
use bevy::hierarchy::Children;
use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::input::ButtonInput;
use bevy::log::debug;
use bevy::prelude::{
    EventReader, KeyCode, MouseButton, Query, Res, ResMut, Style, Text, Val, Visibility, Window,
    With, Without,
};
use bevy::sprite::TextureAtlas;
use bevy::ui::{BorderColor, Interaction};
use bevy::window::PrimaryWindow;
use shared::world::{ItemId, ItemStack};

pub fn render_inventory_hotbar(
    (
        mut text_query,
        mut atlas_query,
        mut floating_stack_query,
        mut cursor_query,
        mut visibility_query,
        window_query,
        mut hotbar_query,
    ): (
        Query<&mut Text>,
        Query<(&mut TextureAtlas, &mut Visibility), Without<InventoryRoot>>,
        Query<(&mut Style, &mut FloatingStack, &Children), With<FloatingStack>>,
        Query<(&Interaction, &mut BorderColor, &InventoryCell, &Children), With<InventoryCell>>,
        Query<&mut Visibility, With<InventoryRoot>>,
        Query<&Window, With<PrimaryWindow>>,
        Query<&mut Hotbar>,
    ),
    (keyboard_input, mouse_input, key_map, mut inventory, materials): (
        Res<ButtonInput<KeyCode>>,
        Res<ButtonInput<MouseButton>>,
        Res<KeyMap>,
        ResMut<Inventory>,
        Res<MaterialResource>,
    ),
    mut scroll: EventReader<MouseWheel>,
) {
    let mut vis = visibility_query.single_mut();
    if is_action_just_pressed(GameAction::ToggleInventory, &keyboard_input, &key_map) {
        *vis = match *vis {
            Visibility::Hidden => Visibility::Visible,
            _ => Visibility::Hidden,
        };
    }

    if is_action_just_pressed(GameAction::DebugGetBlock, &keyboard_input, &key_map) {
        debug!("Blocks given to user");
        inventory.add_item_to_inventory(ItemStack {
            item_id: ItemId::Glass,
            item_type: ItemId::Glass.get_default_type(),
            nb: 64,
        });

        inventory.add_item_to_inventory(ItemStack {
            item_id: ItemId::Poppy,
            item_type: ItemId::Poppy.get_default_type(),
            nb: 64,
        });

        inventory.add_item_to_inventory(ItemStack {
            item_id: ItemId::Dandelion,
            item_type: ItemId::Dandelion.get_default_type(),
            nb: 64,
        });
    }

    let (mut style, mut floating_stack, children) = floating_stack_query.single_mut();
    let mut txt = text_query.get_mut(children[0]).unwrap();
    let (mut stack_atlas, mut stack_vis) = atlas_query.get_mut(children[1]).unwrap();

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

    update_inventory_cell(
        &floating_stack.items,
        &mut txt,
        &mut stack_vis,
        &mut stack_atlas,
        &materials,
    );

    if let Some(c_pos) = window_query.single().cursor_position() {
        style.top = Val::Px(c_pos.y);
        style.left = Val::Px(c_pos.x);
    }

    for (interaction, mut border_color, cell, children) in cursor_query.iter_mut() {
        // Don't update hidden cells, waste of resources
        if cell.id >= MAX_HOTBAR_SLOTS && *vis != Visibility::Visible {
            return;
        }

        let stack = inventory.inner.get(&cell.id).cloned();
        let mut txt: bevy::prelude::Mut<'_, Text> = text_query.get_mut(children[0]).unwrap();
        let (mut stack_atlas, mut stack_vis) = atlas_query.get_mut(children[1]).unwrap();

        update_inventory_cell(
            &stack,
            &mut txt,
            &mut stack_vis,
            &mut stack_atlas,
            &materials,
        );

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
                && stack.unwrap().item_id == floating_items.unwrap().item_id
                && stack.unwrap().nb < stack.unwrap().item_id.get_max_stack()
            {
                let stack = stack.unwrap();
                let floating_items = floating_items.unwrap();
                inventory.add_item_to_stack(
                    cell.id,
                    remove_item_floating_stack(
                        &mut floating_stack,
                        stack.item_id.get_max_stack() - stack.nb,
                    ),
                    floating_items.item_id,
                    stack.item_type,
                );
            } else {
                if stack_exists {
                    let stack = stack.unwrap();
                    floating_stack.items = Some(stack);
                    // If no exchange is made with floating stack, clear cell
                    if !floating_exists {
                        inventory.inner.remove(&cell.id);
                    }
                }

                // Transfer items from floating stack to inventory cell
                if floating_exists {
                    let floating_items = floating_items.unwrap();
                    inventory.inner.insert(cell.id, floating_items);
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

                    if floating_items.item_id == stack.item_id && floating_items.nb > 0 {
                        // Get added nb of items into inventory -> removes them from floating stack

                        remove_item_floating_stack(
                            &mut floating_stack,
                            inventory.add_item_to_stack(
                                cell.id,
                                1,
                                floating_items.item_id,
                                floating_items.item_type,
                            ),
                        );
                    }
                } else if floating_items.nb > 0 {
                    // Get added nb of items into inventory -> removes them from floating stack
                    remove_item_floating_stack(
                        &mut floating_stack,
                        inventory.add_item_to_stack(
                            cell.id,
                            1,
                            floating_items.item_id,
                            floating_items.item_type,
                        ),
                    );
                }
            }
            // Else if hovering a stack : cut hovered stack in half (rounded up), and push it to floating stack
            else if stack_exists {
                let stack = stack.unwrap();
                let nb = (stack.nb + 1) / 2;
                // Get removed nb of items removed from inventory -> adds them into the floating stack
                add_item_floating_stack(
                    &mut floating_stack,
                    inventory.remove_item_from_stack(cell.id, nb),
                    stack.item_id,
                    stack.item_type,
                );
            }
        } else {
            border_color.0 = Color::WHITE;
        }
    }
}

pub fn update_inventory_cell(
    stack: &Option<shared::world::ItemStack>,
    txt: &mut Text,
    visibility: &mut Visibility,
    atlas: &mut TextureAtlas,
    materials: &MaterialResource,
) {
    // Set content
    if let Some(fstack) = stack {
        txt.sections[0].value = format!("{:?}", fstack.nb);
        atlas.index = (materials
            .items
            .uvs
            .get(&format!("{:?}", fstack.item_id))
            .unwrap()
            .u0
            * materials.items.uvs.len() as f32) as usize;
        *visibility = Visibility::Inherited;
    } else {
        txt.sections[0].value = "".to_string();
        *visibility = Visibility::Hidden;
    };
}
