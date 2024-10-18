use crate::input::data::GameAction;
use crate::input::keyboard::is_action_pressed;
use crate::KeyMap;
use bevy::input::ButtonInput;
use bevy::prelude::*;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use bincode::Options;
use shared::messages::{ClientToServerMessage, NetworkPlayerInput, PlayerInputs};

pub fn upload_player_inputs_system(
    mut client: ResMut<RenetClient>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    key_map: Res<KeyMap>,
) {
    let mut actions: Vec<NetworkPlayerInput> = vec![];
    if is_action_pressed(GameAction::MoveBackward, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::Backward)
    }
    if is_action_pressed(GameAction::MoveForward, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::Forward)
    }
    if is_action_pressed(GameAction::MoveLeft, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::Left)
    }
    if is_action_pressed(GameAction::MoveRight, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::Right)
    }
    if is_action_pressed(GameAction::ToggleFlyMode, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::ToggleFlyMode)
    }
    if is_action_pressed(GameAction::FlyUp, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::FlyUp);
    }
    if is_action_pressed(GameAction::FlyDown, &keyboard_input, &key_map) {
        actions.push(NetworkPlayerInput::FlyDown);
    }

    let msg = ClientToServerMessage::PlayerInputs(PlayerInputs {
        tick: 0,
        actions,
        direction: Vec3::ZERO,
    });
    let payload = bincode::options().serialize(&msg).unwrap();
    client.send_message(DefaultChannel::ReliableOrdered, payload);
}
