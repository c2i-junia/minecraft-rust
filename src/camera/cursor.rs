use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::UIMode;
use crate::Player;

// System to hide and lock the cursor
pub fn cursor_grab_system(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn set_cursor_visibility(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    player: Query<&Player>,
) {
    let mut window = windows.single_mut();
    let player = player.single();
    window.cursor.visible = player.ui_mode == UIMode::Opened;
}
