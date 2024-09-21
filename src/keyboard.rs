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
    Escape,
    ToggleFps,
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
        map.insert(GameAction::Escape, vec![KeyCode::Escape]);
        map.insert(GameAction::ToggleFps, vec![KeyCode::F3]);
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

pub fn get_action_keys(action: GameAction) -> Vec<KeyCode> {
    KEY_MAP.get(&action).unwrap().to_vec()
}
