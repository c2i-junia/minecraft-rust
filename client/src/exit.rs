use crate::keyboard::{is_action_just_pressed, GameAction};
use bevy::app::AppExit;
use bevy::input::ButtonInput;
use bevy::prelude::{EventWriter, KeyCode, Res};

pub fn exit_system(mut exit: EventWriter<AppExit>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if is_action_just_pressed(GameAction::Escape, &keyboard_input) {
        exit.send(AppExit::Success);
    }
}
