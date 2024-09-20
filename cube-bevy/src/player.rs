use crate::keyboard::*;
use crate::CameraController;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

// System to move the player based on keyboard input
pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    camera_query: Query<&Transform, (With<Camera>, With<CameraController>, Without<Player>)>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Camera>)>,
) {
    let mut player_transform = player_query.single_mut();
    let camera_transform = camera_query.single();

    let speed = 5.0;

    // Calculate movement directions relative to the camera
    let mut forward = camera_transform.forward();
    forward.y = 0.0;
    forward = forward.normalize();

    let mut right = camera_transform.right();
    right.y = 0.0;
    right = right.normalize();

    let mut direction = Vec3::ZERO;

    // Adjust direction based on key presses
    if is_action_pressed(GameAction::MoveBackward, &keyboard_input) {
        direction -= forward;
    }
    if is_action_pressed(GameAction::MoveForward, &keyboard_input) {
        direction += forward;
    }
    if is_action_pressed(GameAction::MoveLeft, &keyboard_input) {
        direction -= right;
    }
    if is_action_pressed(GameAction::MoveRight, &keyboard_input) {
        direction += right;
    }

    // Move the player
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        player_transform.translation += direction * speed * time.delta_seconds();
    }
}
