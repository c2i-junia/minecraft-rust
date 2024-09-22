use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

// System to hide and lock the cursor
pub fn cursor_grab_system(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}
