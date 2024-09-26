use crate::input::keyboard::{is_action_just_pressed, GameAction};
use bevy::pbr::wireframe::WireframeConfig;
use bevy::prelude::*;

#[derive(Resource)]
pub struct BlockDebugWireframeSettings {
    pub is_enabled: bool,
}

pub fn toggle_wireframe_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut settings: ResMut<BlockDebugWireframeSettings>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut config: ResMut<WireframeConfig>,
) {
    if is_action_just_pressed(GameAction::ToggleBlockWireframeDebugMode, &keyboard_input)
        && !settings.is_enabled
    {
        settings.is_enabled = true;
        config.global = true;
        for (_, material) in materials.iter_mut() {
            material.alpha_mode = AlphaMode::Blend;
            material.base_color.set_alpha(0.3);
        }
        return;
    }

    if is_action_just_pressed(GameAction::ToggleBlockWireframeDebugMode, &keyboard_input) {
        settings.is_enabled = false;
        config.global = false;
        for (_, material_handle) in materials.iter_mut() {
            material_handle.base_color.set_alpha(1.0);
        }
    }
}
