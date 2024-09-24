use crate::camera::CameraController;
use crate::input::keyboard::*;
use crate::player::{Player, ViewMode};
use crate::world::load_chunk_around_player;
use bevy::prelude::*;

fn is_block_at_position(
    position: Vec3,
    blocks: &Query<&Transform, (Without<Player>, Without<Camera>)>,
) -> bool {
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

fn check_player_collision(
    player_position: Vec3,
    blocks: &Query<&Transform, (Without<Player>, Without<Camera>)>,
) -> bool {
    let player_width = 0.4;
    let player_height = 1.8;

    // Vérification de la collision avec les pieds et la tête du joueur
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

    // On vérifie les coins du joueur
    let offsets = [
        Vec3::new(-player_width, 0.0, -player_width), // bas gauche devant
        Vec3::new(player_width, 0.0, -player_width),  // bas droite devant
        Vec3::new(-player_width, 0.0, player_width),  // bas gauche derrière
        Vec3::new(player_width, 0.0, player_width),   // bas droite derrière
    ];

    // Vérifier la collision au niveau des pieds
    for offset in &offsets {
        let check_pos = foot_position + *offset;
        if is_block_at_position(check_pos, blocks) {
            return true;
        }
    }

    // Vérifier la collision au niveau de la tête
    for offset in &offsets {
        let check_pos = head_position + *offset;
        if is_block_at_position(check_pos, blocks) {
            return true;
        }
    }

    false
}

// System to move the player based on keyboard input
pub fn player_movement_system(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &mut Player, &mut Handle<StandardMaterial>)>,
    camera_query: Query<&Transform, (With<Camera>, With<CameraController>, Without<Player>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    blocks: Query<&Transform, (Without<Player>, Without<Camera>)>,
) {
    if is_action_just_pressed(GameAction::ToggleViewMode, &keyboard_input) {
        for (_, mut player, _) in player_query.iter_mut() {
            // TOFIX: there is only one player so no need to iterate ??
            player.toggle_view_mode();
        }
    }

    if is_action_just_pressed(GameAction::ToggleChunkDebugMode, &keyboard_input) {
        for (_, mut player, _) in player_query.iter_mut() {
            player.toggle_chunk_debug_mode();
        }
    }

    // fly mode (f key)
    if is_action_just_pressed(GameAction::ToggleFlyMode, &keyboard_input) {
        for (_, mut player, _) in player_query.iter_mut() {
            player.toggle_fly_mode();
        }
    }

    let (mut player_transform, mut player, material_handle_mut_ref) = player_query.single_mut();
    let camera_transform = camera_query.single();

    load_chunk_around_player(
        player_transform.translation,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    let material_handle = &*material_handle_mut_ref;

    match player.view_mode {
        ViewMode::FirstPerson => {
            // make player transparent
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

    let speed;
    if player.is_flying {
        speed = 15.0;
    } else {
        speed = 5.0;
    }

    let gravity = (-9.8) * 4.0;
    let jump_velocity = 6.0 * 2.0;

    // flying mode
    if player.is_flying {
        if is_action_pressed(GameAction::FlyUp, &keyboard_input) {
            player_transform.translation.y += speed * 2.0 * time.delta_seconds();
        }
        if is_action_pressed(GameAction::FlyDown, &keyboard_input) {
            player_transform.translation.y -= speed * 2.0 * time.delta_seconds();
        }
    }

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

    // Move the player (xy plane only), only if there is no blocks
    if direction.length_squared() > 0.0 {
        direction = direction.normalize();

        // Déplacement sur l'axe X
        let new_pos_x = player_transform.translation
            + Vec3::new(direction.x, 0.0, 0.0) * speed * time.delta_seconds();
        if !check_player_collision(new_pos_x, &blocks) {
            player_transform.translation.x = new_pos_x.x;
        }

        // Déplacement sur l'axe Z
        let new_pos_z = player_transform.translation
            + Vec3::new(0.0, 0.0, direction.z) * speed * time.delta_seconds();
        if !check_player_collision(new_pos_z, &blocks) {
            player_transform.translation.z = new_pos_z.z;
        }
    }

    // Handle jumping (if on the ground) and gravity (only if not flying)
    if !player.is_flying {
        if player.on_ground && is_action_pressed(GameAction::Jump, &keyboard_input) {
            // Player can jump only when grounded
            player.vertical_velocity = jump_velocity;
            player.on_ground = false;
        } else if !player.on_ground {
            // Apply gravity when the player is in the air
            player.vertical_velocity += gravity * time.delta_seconds();
        }
    }

    // apply gravity and verify vertical collisions
    let new_y = player_transform.translation.y + player.vertical_velocity * time.delta_seconds();

    // Vérifier uniquement les collisions verticales (sol et plafond)
    if check_player_collision(
        Vec3::new(
            player_transform.translation.x,
            new_y,
            player_transform.translation.z,
        ),
        &blocks,
    ) {
        // Si un bloc est détecté sous le joueur, il reste sur le bloc
        player.on_ground = true;
        player.vertical_velocity = 0.0; // Réinitialiser la vélocité verticale si le joueur est au sol
    } else {
        // Si aucun bloc n'est détecté sous le joueur, il continue de tomber
        player_transform.translation.y = new_y;
        player.on_ground = false;
    }

    // If the player is below the world, reset their position
    if player_transform.translation.y < -50.0 {
        player_transform.translation = Vec3::new(0.0, 100.0, 0.0);
        player.vertical_velocity = 0.0;
    }
}
