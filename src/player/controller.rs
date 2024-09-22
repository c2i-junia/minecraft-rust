use crate::camera::CameraController;
use crate::input::keyboard::*;
use crate::player::{Player, ViewMode};
use bevy::prelude::*;

// function to check if there's a block at a given position
fn is_block_at_position(position: Vec3, blocks: &Query<&Transform, Without<Player>>) -> bool {
    for block_transform in blocks.iter() {
        let block_pos = block_transform.translation;
        // consider a margin of 0.5 (as each block is centered at a whole position)
        if (block_pos.x - position.x).abs() < 0.5
            && (block_pos.y - position.y).abs() < 0.5
            && (block_pos.z - position.z).abs() < 0.5
        {
            return true;
        }
    }
    false
}

// checks player collisions with blocks, taking player size into account
fn check_player_collision(
    player_position: Vec3,
    blocks: &Query<&Transform, Without<Player>>,
) -> bool {
    // the player is 1 block wide (X and Z) and 2 blocks tall (Y)
    let player_width = 0.5;
    let player_height = 2.0;

    // check points at the player's feet (base) and head (top)
    let foot_position = Vec3::new(
        player_position.x,
        player_position.y - player_height / 2.0,
        player_position.z,
    );
    let head_position = Vec3::new(
        player_position.x,
        player_position.y + player_height / 2.0,
        player_position.z,
    );

    // check the corners of the player's collision box (bottom and top)
    let offsets = [
        Vec3::new(-player_width, 0.0, -player_width), // bottom left front corner
        Vec3::new(player_width, 0.0, -player_width),  // bottom right front corner
        Vec3::new(-player_width, 0.0, player_width),  // bottom left back corner
        Vec3::new(player_width, 0.0, player_width),   // bottom right back corner
    ];

    // check collisions for all bottom corners (feet)
    for offset in &offsets {
        let check_pos = foot_position + *offset;
        if is_block_at_position(check_pos, blocks) {
            return true;
        }
    }

    // check collisions for all top corners (head)
    for offset in &offsets {
        let check_pos = head_position + *offset;
        if is_block_at_position(check_pos, blocks) {
            return true;
        }
    }

    false
}

// player movement system
pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Handle<StandardMaterial>)>,
    camera_query: Query<&Transform, (With<Camera>, With<CameraController>, Without<Player>)>,
    blocks: Query<&Transform, Without<Player>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    if is_action_just_pressed(GameAction::Escape, &keyboard_input) {
        std::process::exit(0);
    }

    if is_action_just_pressed(GameAction::ToggleViewMode, &keyboard_input) {
        for (_, mut player, _) in player_query.iter_mut() {
            player.toggle_view_mode();
        }
    }

    let (mut player_transform, mut player, material_handle_mut_ref) = player_query.single_mut();
    let camera_transform = camera_query.single();

    let material_handle = &*material_handle_mut_ref;

    match player.view_mode {
        ViewMode::FirstPerson => {
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = Color::srgba(0.0, 0.0, 0.0, 0.0);
            }
        }
        ViewMode::ThirdPerson => {
            if let Some(material) = materials.get_mut(material_handle) {
                material.base_color = Color::srgba(1.0, 0.0, 0.0, 1.0);
            }
        }
    }

    let speed = 5.0;
    let gravity = (-9.8) * 4.0;
    let jump_velocity = 5.0 * 2.0;

    // horizontal movement
    let mut forward = camera_transform.forward().xyz();
    forward.y = 0.0; // ignore vertical movement for horizontal movement

    let mut right = camera_transform.right().xyz();
    right.y = 0.0; // same here

    let mut direction = Vec3::ZERO;

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

    if direction.length_squared() > 0.0 {
        direction = direction.normalize(); // normalize the direction vector for uniform movement

        // calculate new positions separately for each axis
        let new_pos_x = player_transform.translation
            + Vec3::new(direction.x, 0.0, 0.0) * speed * time.delta_seconds();
        let new_pos_z = player_transform.translation
            + Vec3::new(0.0, 0.0, direction.z) * speed * time.delta_seconds();

        // check for collisions in the X axis movement
        if !check_player_collision(new_pos_x, &blocks) {
            // no collision in X: allow movement in X
            player_transform.translation.x = new_pos_x.x;
        }

        // check for collisions in the Z axis movement
        if !check_player_collision(new_pos_z, &blocks) {
            // no collision in Z: allow movement in Z
            player_transform.translation.z = new_pos_z.z;
        }
    }

    // gravity and jump
    if player.on_ground && is_action_pressed(GameAction::Jump, &keyboard_input) {
        player.vertical_velocity = jump_velocity;
        player.on_ground = false;
    } else if !player.on_ground {
        player.vertical_velocity += gravity * time.delta_seconds();
    }

    let new_y = player_transform.translation.y + player.vertical_velocity * time.delta_seconds();

    // check for collisions in vertical (Y) movement
    if check_player_collision(
        Vec3::new(
            player_transform.translation.x,
            new_y,
            player_transform.translation.z,
        ),
        &blocks,
    ) {
        // collision detected in Y (ground or ceiling)
        player.vertical_velocity = 0.0;
        player.on_ground = true;
    } else {
        // no collision, apply vertical movement
        player_transform.translation.y = new_y;
        player.on_ground = false;
    }

    // reset player position if they fall below the world
    if player_transform.translation.y < -10.0 {
        player_transform.translation = Vec3::new(0.0, 1.0, 0.0);
        player.on_ground = true;
        player.vertical_velocity = 0.0;
    }
}
