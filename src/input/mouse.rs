use crate::camera::*;
use crate::constants::CUBE_SIZE;
use crate::materials::MaterialResource;
use crate::player::inventory::*;
use crate::player::Player;
use crate::world::{Block, WorldMap};
use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

// Helper function to snap a Vec3 position to the grid
fn snap_to_grid(position: Vec3) -> Vec3 {
    Vec3::new(position.x.round(), position.y.round(), position.z.round())
}

// Function to handle block placement and breaking
pub fn handle_block_interactions(
    mut player: Query<&mut Player>,
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>, // to handle mouse input
    mut meshes: ResMut<Assets<Mesh>>,           // for adding new block meshes
    raycast_source: Query<&RaycastSource<BlockRaycastSet>>, // raycast from the camera
    mut world_map: ResMut<WorldMap>,
    material_resource: Res<MaterialResource>,
) {
    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE))); // Utilisez CUBE_SIZE ici

    let raycast_source = raycast_source.single();

    // Handle left-click for breaking blocks
    if mouse_input.just_pressed(MouseButton::Left) {
        // Check if there are any intersections with a block
        if let Some((entity, _intersection)) = raycast_source.intersections().first() {
            // Remove the hit block
            commands.entity(*entity).despawn();

            // add the block to the player's inventory
            add_item_to_inventory(&mut player, 1, 1);
        }
    }

    // Handle right-click for placing blocks
    if mouse_input.just_pressed(MouseButton::Right) {
        if let Some((_entity, intersection)) = raycast_source.intersections().first() {
            // Check if the block is in the player's inventory
            if has_item(&mut player, 1) {
                // Remove the block from the player's inventory
                remove_item_from_inventory(&mut player, 1, 1);
            } else {
                return;
            }

            // Get the normal of the face where the block will be placed
            let normal = intersection.normal(); // This is already a Vec3, no need to unwrap
                                                // Calculate the block position by adding a small offset to the intersection point
            let mut position = intersection.position() + normal * 0.51;

            // Snap the position to the grid
            position = snap_to_grid(position);

            world_map.set_block(
                position.x as i32,
                position.y as i32,
                position.z as i32,
                Block::Dirt,
                &mut commands,
                cube_mesh.clone(),
                &material_resource,
            );
        }
    }
}
