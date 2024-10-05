use std::{
    collections::BTreeMap,
    fs::{self, File},
    io::Write,
    path::Path,
};

use bevy::{
    input::ButtonInput,
    prelude::{Component, KeyCode, Res},
};
use ron::{from_str, ser::PrettyConfig};
use serde::{Deserialize, Serialize};

use crate::KeyMap;

#[derive(
    Eq, Hash, PartialEq, Component, Debug, Clone, Copy, Serialize, Deserialize, PartialOrd, Ord,
)]
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
    OpenChat,
    RenderDistanceMinus,
    RenderDistancePlus,
}

pub fn is_action_pressed(
    action: GameAction,
    keyboard_input: &ButtonInput<KeyCode>,
    key_map: &KeyMap,
) -> bool {
    if let Some(key_codes) = key_map.map.get(&action) {
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
    keyboard_input: &ButtonInput<KeyCode>,
    key_map: &KeyMap,
) -> bool {
    if let Some(key_codes) = key_map.map.get(&action) {
        for key_code in key_codes {
            if keyboard_input.just_pressed(*key_code) {
                return true;
            }
        }
    }
    false
}

pub fn is_action_just_released(
    action: GameAction,
    keyboard_input: &ButtonInput<KeyCode>,
    key_map: &KeyMap,
) -> bool {
    if let Some(key_codes) = key_map.map.get(&action) {
        for key_code in key_codes {
            if keyboard_input.just_released(*key_code) {
                return true;
            }
        }
    }
    false
}

pub fn get_action_keys(action: GameAction, key_map: &KeyMap) -> Vec<KeyCode> {
    key_map.map.get(&action).unwrap().to_vec()
}

const BINDS_PATH: &str = "keybinds.ron";

pub fn get_keybinds() -> KeyMap {
    // Try to get & serialize existing binds
    if let Ok(content) = fs::read_to_string(Path::new(BINDS_PATH)) {
        if let Ok(key_map) = from_str::<KeyMap>(&content) {
            return key_map;
        }
    }

    // If binds cannot be loaded, get default ones
    KeyMap {
        map: {
            let mut map = BTreeMap::new();
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
            map.insert(GameAction::OpenChat, vec![KeyCode::KeyT]);
            map.insert(GameAction::RenderDistanceMinus, vec![KeyCode::KeyO]);
            map.insert(GameAction::RenderDistancePlus, vec![KeyCode::KeyP]);
            map
        },
    }
}

pub fn save_keybinds(key_map: Res<KeyMap>) {
    let pretty_config = PrettyConfig::new()
        .with_depth_limit(3)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);
    if let Ok(serialized) = ron::ser::to_string_pretty(key_map.into_inner(), pretty_config) {
        if let Ok(mut file) = File::create(Path::new(BINDS_PATH)) {
            if let Err(e) = file.write_all(serialized.as_bytes()) {
                println!("Error while saving keybinds : {}", e);
            } else {
                println!("Key binds saved");
            }
        }
    }
}
