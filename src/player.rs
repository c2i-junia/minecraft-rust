use crate::keyboard::*;
use crate::CameraController;
use bevy::prelude::*;

#[derive(Component)]
pub struct Player {
    pub vertical_velocity: f32,
    pub on_ground: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            vertical_velocity: 0.0,
            on_ground: true,
        }
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 2.0, 1.0))),
            material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .insert(Player::new());
}

// System to move the player based on keyboard input
pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    camera_query: Query<&Transform, (With<Camera>, With<CameraController>, Without<Player>)>,
) {
    if is_action_pressed(GameAction::Escape, &keyboard_input) {
        std::process::exit(0);
    }

    let (mut player_transform, mut player) = player_query.single_mut();
    let camera_transform = camera_query.single();

    let speed = 5.0;
    let gravity = (-9.8) * 4.0; 
    let jump_velocity = 5.0 * 2.0;

    // Calculate movement directions relative to the camera
    let mut forward = camera_transform.forward().xyz();
    forward.y = 0.0;

    let mut right = camera_transform.right().xyz();
    right.y = 0.0;

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

    // Move the player (xy plane only)
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();
        player_transform.translation += direction * speed * time.delta_seconds();
    }

    // Handle jumping (if on the ground) and gravity
    if player.on_ground && is_action_pressed(GameAction::Jump, &keyboard_input) {
        // Player can jump only when grounded
        player.vertical_velocity = jump_velocity;
        player.on_ground = false;
    } else if !player.on_ground {
        // Apply gravity when the player is in the air
        player.vertical_velocity += gravity * time.delta_seconds();
    }

    // Apply vertical velocity
    player_transform.translation.y += player.vertical_velocity * time.delta_seconds();

    // Ground detection: If the player is below or touching the platform, stop vertical movement
    let is_on_plateform: bool = player_transform.translation.x < 11.0 
        && player_transform.translation.x > -11.0
        && player_transform.translation.z < 11.0 
        && player_transform.translation.z > -11.0;

    if player_transform.translation.y <= 1.0 && is_on_plateform {
        player_transform.translation.y = 1.0;
        player.vertical_velocity = 0.0;
        player.on_ground = true;
    } else {
        player.on_ground = false;
    }

    // Detect if the player falls off the platform
    println!(
        "Player coordinates: x = {:.2}, y = {:.2}, z = {:.2}",
        player_transform.translation.x,
        player_transform.translation.y,
        player_transform.translation.z
    );
    if player_transform.translation.y < -10.0 {
        println!("Player fell off the platform!");
        player_transform.translation = Vec3::new(0.0, 1.0, 0.0);  // Reset position
        player.vertical_velocity = 0.0;
        player.on_ground = true;
    }
}
