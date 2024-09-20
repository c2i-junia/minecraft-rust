use bevy::{
    input::Input,
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
}

lazy_static! {
    static ref KEY_MAP: HashMap<GameAction, Vec<KeyCode>> = {
        let mut map = HashMap::new();
        map.insert(GameAction::MoveForward, vec![KeyCode::W, KeyCode::Up]);
        map.insert(GameAction::MoveBackward, vec![KeyCode::S, KeyCode::Down]);
        map.insert(GameAction::MoveLeft, vec![KeyCode::A, KeyCode::Left]);
        map.insert(GameAction::MoveRight, vec![KeyCode::D, KeyCode::Right]);
        map
    };
}

pub fn is_action_pressed(action: GameAction, keyboard_input: &Res<Input<KeyCode>>) -> bool {
    if let Some(key_codes) = KEY_MAP.get(&action) {
        for key_code in key_codes {
            if keyboard_input.pressed(*key_code) {
                return true;
            }
        }
    }
    false
}
