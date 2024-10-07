use crate::ui::UIMode;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn set_mouse_visibility(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    ui_mode: Res<UIMode>,
) {
    let mut window = windows.single_mut();
    window.cursor.visible = *ui_mode == UIMode::Opened;
}
