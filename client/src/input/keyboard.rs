use bevy::{
    input::ButtonInput,
    prelude::{KeyCode, Res},
};
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
pub enum GameAction {
    MoveForward,
    MoveBackward,
    MoveLeft,
    MoveRight,
    Jump,
    Escape,
    ToggleFps,
    ToggleViewMode,
    ToggleChunkDebugMode,
    ToggleFlyMode,
    FlyUp,
    FlyDown,
    ToggleBlockWireframeDebugMode,
    ToggleInventory,
    RenderDistanceMinus,
    RenderDistancePlus,
}

lazy_static! {
    static ref KEY_MAP: HashMap<GameAction, Vec<KeyCode>> = {
        let mut map = HashMap::new();
        map.insert(
            GameAction::MoveForward,
            vec![KeyCode::KeyW, KeyCode::ArrowUp],
        );
        map.insert(
            GameAction::MoveBackward,
            vec![KeyCode::KeyS, KeyCode::ArrowDown],
        );
        map.insert(
            GameAction::MoveLeft,
            vec![KeyCode::KeyA, KeyCode::ArrowLeft],
        );
        map.insert(
            GameAction::MoveRight,
            vec![KeyCode::KeyD, KeyCode::ArrowRight],
        );
        map.insert(GameAction::Jump, vec![KeyCode::Space]);
        map.insert(GameAction::Escape, vec![KeyCode::Escape]);
        map.insert(GameAction::ToggleFps, vec![KeyCode::F3]);
        map.insert(GameAction::ToggleViewMode, vec![KeyCode::F5]);
        map.insert(GameAction::ToggleChunkDebugMode, vec![KeyCode::F4]);
        map.insert(GameAction::ToggleFlyMode, vec![KeyCode::KeyF]);
        map.insert(GameAction::FlyUp, vec![KeyCode::Space]);
        map.insert(GameAction::FlyDown, vec![KeyCode::ShiftLeft]);
        map.insert(GameAction::ToggleBlockWireframeDebugMode, vec![KeyCode::F6]);
        map.insert(GameAction::ToggleInventory, vec![KeyCode::KeyE]);
        map.insert(GameAction::RenderDistanceMinus, vec![KeyCode::KeyO]);
        map.insert(GameAction::RenderDistancePlus, vec![KeyCode::KeyP]);
        map
    };
}

pub fn is_action_pressed(action: GameAction, keyboard_input: &Res<ButtonInput<KeyCode>>) -> bool {
    if let Some(key_codes) = KEY_MAP.get(&action) {
        for key_code in key_codes {
            if keyboard_input.pressed(*key_code) {
                return true;
            }
        }
    }
    false
}

pub fn is_action_just_pressed(
    action: GameAction,
    keyboard_input: &Res<ButtonInput<KeyCode>>,
) -> bool {
    if let Some(key_codes) = KEY_MAP.get(&action) {
        for key_code in key_codes {
            if keyboard_input.just_pressed(*key_code) {
                return true;
            }
        }
    }
    false
}

pub fn get_action_keys(action: GameAction) -> Vec<KeyCode> {
    KEY_MAP.get(&action).unwrap().to_vec()
}
