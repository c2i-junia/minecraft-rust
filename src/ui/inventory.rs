use crate::{
    constants::{MAX_ITEM_SLOTS, MAX_ITEM_STACK},
    inventory,
    items::{Item, ItemsType},
    keyboard::{get_action_keys, GameAction},
    player::Player,
    MaterialResource,
};
use bevy::{
    prelude::*, render::texture::TRANSPARENT_IMAGE_HANDLE, ui::FocusPolicy, window::PrimaryWindow,
};

use super::BaseUiDialog;

/// Marker for Inventory root
#[derive(Component)]
pub struct InventoryRoot;

/// Main inventory dialog
#[derive(Component)]
pub struct InventoryDialog;

#[derive(Component)]
pub struct InventoryCell {
    id: u32,
}

/// The current selected stack, not considered in the player's inventory
#[derive(Component)]
pub struct FloatingStack {
    items: Option<Item>,
}

#[derive(Component)]
pub struct InventoryText;

pub fn setup_inventory(mut commands: Commands) {
    // Inventory root : root container for the inventory
    let root = commands
        .spawn((
            BaseUiDialog,
            InventoryRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.4)),
                // Z-index of 1 : displayed above game, but under everything else
                z_index: ZIndex::Global(1),
                visibility: Visibility::Hidden,
                style: Style {
                    position_type: PositionType::Absolute,
                    // Cover whole screen as a dark backdrop
                    left: Val::Percent(0.),
                    right: Val::Percent(0.),
                    bottom: Val::Percent(0.),
                    top: Val::Percent(0.),
                    // Align children at its center
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let dialog = commands
        .spawn((
            InventoryDialog,
            NodeBundle {
                background_color: BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                border_radius: BorderRadius::all(Val::Percent(10.)),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Percent(7.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let inventory_title = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Inventory",
                TextStyle {
                    font_size: 24.,
                    ..Default::default()
                },
            ),
            style: Style {
                align_content: AlignContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let inventory_grid = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::auto(9),
                margin: UiRect::all(Val::Px(20.)),
                border: UiRect::all(Val::Px(1.)),
                position_type: PositionType::Relative,
                ..Default::default()
            },
            border_color: BorderColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {
            for i in 0..MAX_ITEM_SLOTS {
                builder
                    .spawn((
                        InventoryCell { id: i },
                        ButtonBundle {
                            border_color: BorderColor(Color::BLACK),
                            focus_policy: FocusPolicy::Block,
                            style: Style {
                                width: Val::Px(50.),
                                height: Val::Px(50.),
                                margin: UiRect::ZERO,
                                padding: UiRect::all(Val::Percent(10.)),
                                border: UiRect::all(Val::Px(1.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    ))
                    .with_children(|btn| {
                        btn.spawn(TextBundle {
                            text: Text::from_section(
                                "",
                                TextStyle {
                                    font_size: 15.,
                                    ..Default::default()
                                },
                            ),
                            style: Style {
                                position_type: PositionType::Absolute,
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                        btn.spawn(ImageBundle {
                            z_index: ZIndex::Local(-1),
                            style: Style {
                                //     left: Val::Percent(5.),
                                //     right: Val::Percent(5.),
                                //     bottom: Val::Percent(15.),
                                //     top: Val::Percent(15.),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    });
            }
        })
        .id();

    let floating_stack = commands
        .spawn((
            FloatingStack { items: None },
            NodeBundle {
                focus_policy: FocusPolicy::Pass,
                style: Style {
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 15.,
                    ..Default::default()
                },
            ));
            btn.spawn(ImageBundle {
                z_index: ZIndex::Local(-1),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.),
                    right: Val::Percent(0.),
                    bottom: Val::Percent(0.),
                    top: Val::Percent(0.),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id();

    commands
        .entity(dialog)
        .push_children(&[inventory_title, inventory_grid]);

    commands
        .entity(root)
        .push_children(&[dialog, floating_stack]);
}

// Open inventory when E key is pressed
pub fn toggle_inventory(
    mut q: Query<&mut Visibility, With<InventoryRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    let keys = get_action_keys(GameAction::ToggleInventory);
    for key in keys {
        if kbd.just_pressed(key) {
            let mut vis = q.single_mut();
            *vis = match *vis {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}

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

pub fn inventory_update_system(
    player_query: Query<&Player>,
    mut btn_query: Query<&Children, With<InventoryCell>>,
    mut text_query: Query<&mut Text>,
    mut image_query: Query<&mut UiImage>,
    mut floating_stack_query: Query<(&mut Style, &FloatingStack, &Children), With<FloatingStack>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    vis: Query<&mut Visibility, With<InventoryRoot>>,
    material_resource: Res<MaterialResource>,
) {
    // If inventory is hidden, do not update it
    if vis.single() == Visibility::Hidden {
        return;
    }

    let player = player_query.single();

    // For each cell : Update content
    for (children, i) in btn_query.iter_mut().zip(0..MAX_ITEM_SLOTS) {
        let stack = player.inventory.get(&i);
        let mut txt = text_query.get_mut(children[0]).unwrap();
        let mut img = image_query.get_mut(children[1]).unwrap();

        // Set content
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
    }

    let (mut style, fstack, children) = floating_stack_query.single_mut();
    let mut txt = text_query.get_mut(children[0]).unwrap();
    let mut img = image_query.get_mut(children[1]).unwrap();

    // Set content
    if fstack.items.is_none() {
        txt.sections[0].value = "".to_string();
        img.texture = TRANSPARENT_IMAGE_HANDLE;
    } else {
        let fstack = fstack.items.unwrap();
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
                inventory::add_item_to_stack(
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
                            inventory::add_item_to_stack(
                                &mut player,
                                floating_items.id,
                                cell.id,
                                1,
                            ),
                        );
                    }
                } else if floating_items.nb > 0 {
                    // Get added nb of items into inventory -> removes them from floating stack
                    remove_item_floating_stack(
                        &mut floating_stack,
                        inventory::add_item_to_stack(&mut player, floating_items.id, cell.id, 1),
                    );
                }
            }
            // Else if hovering a stack : cut hovered stack in half, and push it to floating stack
            else if stack_exists {
                let stack = *stack.unwrap();
                let nb = stack.nb / 2;
                // Get removed nb of items removed from inventory -> adds them into the floating stack
                add_item_floating_stack(
                    &mut floating_stack,
                    inventory::remove_item_from_stack(&mut player, stack.id, cell.id, nb),
                    stack.id,
                );
            }
        } else {
            border_color.0 = Color::WHITE;
        }
    }
}

/// Removes a number of items from the floating stack\
/// Cannot go lower than 0 items
/// Returns number of items actually removed
pub fn remove_item_floating_stack(floating_stack: &mut FloatingStack, nb: u32) -> u32 {
    if let Some(mut item) = floating_stack.items {
        if nb >= item.nb {
            floating_stack.items = None;
            return item.nb;
        }
        item.nb -= nb;
        floating_stack.items = Some(item);
        return nb;
    }
    0
}

/// Adds a number of items to the floating stack\
/// Cannot go higher than MAX_ITEM_STACK items
/// Returns number of items actually added
pub fn add_item_floating_stack(
    floating_stack: &mut FloatingStack,
    mut nb: u32,
    item_type: ItemsType,
) -> u32 {
    if let Some(mut item) = floating_stack.items {
        if nb + item.nb > MAX_ITEM_STACK {
            nb = MAX_ITEM_STACK - item.nb;
        }
        item.nb += nb;
        nb
    } else {
        if nb > MAX_ITEM_STACK {
            nb = MAX_ITEM_STACK;
        }
        floating_stack.items = Some(Item { id: item_type, nb });
        nb
    }
}
