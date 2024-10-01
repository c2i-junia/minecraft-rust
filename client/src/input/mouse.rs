use crate::UIMode;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

// System to hide and lock the cursor
pub fn mouse_grab_system(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

pub fn set_mouse_visibility(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    ui_mode: Res<UIMode>
) {
    let mut window = windows.single_mut();
    window.cursor.visible = *ui_mode == UIMode::Opened;
}
