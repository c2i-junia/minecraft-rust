use crate::constants::{MAX_HOTBAR_SLOTS, MAX_INVENTORY_SLOTS};
use crate::input::keyboard::{get_action_keys, GameAction};
use crate::keyboard::keyboard_clear_input;
use crate::player::Player;
use crate::ui::{FloatingStack, InventoryCell, InventoryRoot};
use crate::world::MaterialResource;
use bevy::hierarchy::Children;
use bevy::input::ButtonInput;
use bevy::prelude::{
    KeyCode, Query, Res, ResMut, Style, Text, UiImage, Val, Visibility, Window, With,
};
use bevy::render::texture::TRANSPARENT_IMAGE_HANDLE;
use bevy::window::PrimaryWindow;

pub fn inventory_update_system(
    player_query: Query<&Player>,
    mut inventory_query: Query<&Children, With<InventoryCell>>,
    mut text_query: Query<&mut Text>,
    mut image_query: Query<&mut UiImage>,
    mut floating_stack_query: Query<(&mut Style, &FloatingStack, &Children), With<FloatingStack>>,
    mut keyboard_input: ResMut<ButtonInput<KeyCode>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut visibility: Query<&mut Visibility, With<InventoryRoot>>,
    material_resource: Res<MaterialResource>,
) {
    let mut vis = visibility.single_mut();
    let keys = get_action_keys(GameAction::ToggleInventory);
    for key in keys {
        if keyboard_input.just_pressed(key) {
            *vis = match *vis {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }

    if *vis == Visibility::Hidden {
        return;
    }

    let player = player_query.single();

    // For each cell : Update content
    for (children, i) in inventory_query
        .iter_mut()
        .zip(MAX_HOTBAR_SLOTS..MAX_INVENTORY_SLOTS)
    {
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

    keyboard_clear_input(&mut keyboard_input);
}
