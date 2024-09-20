use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use keyboard::*;
use world::*;

mod keyboard;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_world)
        .add_startup_system(cursor_grab_system)
        .add_system(player_movement_system)
        .add_system(camera_control_system)
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct CameraController {
    distance: f32,
    angle_x: f32,
    angle_y: f32,
    mouse_sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            distance: 10.0,
            angle_x: 0.0,
            angle_y: 20.0f32.to_radians(),
            mouse_sensitivity: 0.003,
        }
    }
}

// System to hide and lock the cursor
fn cursor_grab_system(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

// System to control the camera based on mouse movement
fn camera_control_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<
        (&mut Transform, &mut CameraController),
        (With<Camera>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
    let window = windows.single();

    // Skip if the window is not focused
    if !window.focused {
        return;
    }

    let mut delta = Vec2::ZERO;

    // Accumulate mouse motion events
    for event in mouse_motion_events.iter() {
        delta += event.delta;
    }

    for (mut transform, mut controller) in camera_query.iter_mut() {
        controller.angle_x -= delta.x * controller.mouse_sensitivity;
        controller.angle_y += delta.y * controller.mouse_sensitivity;

        // Clamp vertical angle to prevent flipping
        controller.angle_y = controller
            .angle_y
            .clamp(-89.0f32.to_radians(), 89.0f32.to_radians());

        let player_transform = player_query.single();
        let player_position = player_transform.translation;

        // Calculate new camera position
        let x = player_position.x
            + controller.distance * controller.angle_y.cos() * controller.angle_x.sin();
        let y = player_position.y + controller.distance * controller.angle_y.sin();
        let z = player_position.z
            + controller.distance * controller.angle_y.cos() * controller.angle_x.cos();

        let camera_position = Vec3::new(x, y, z);

        // Update camera transform
        *transform =
            Transform::from_translation(camera_position).looking_at(player_position, Vec3::Y);
    }
}

// System to move the player based on keyboard input
fn player_movement_system(
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
