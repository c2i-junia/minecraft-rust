use crate::camera::*;
use crate::constants::{CUBE_SIZE, INTERACTION_DISTANCE};
use crate::player::inventory::*;
use crate::player::Player;
use crate::world::{Block, WorldMap};
use bevy::math::NormedVectorSpace;
use crate::world::WorldRenderRequestUpdateEvent;
use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

// Helper function to snap a Vec3 position to the grid
fn snap_to_grid(position: Vec3) -> Vec3 {
    Vec3::new(position.x.round(), position.y.round(), position.z.round())
}

// Function to handle block placement and breaking
pub fn handle_block_interactions(
    mut player: Query<&mut Player>,
    mut p_transform: Query<&mut Transform, With<Player>>,
    mouse_input: Res<ButtonInput<MouseButton>>, // to handle mouse input
    raycast_source: Query<&RaycastSource<BlockRaycastSet>>, // raycast from the camera
    mut world_map: ResMut<WorldMap>,
    mut commands: Commands,
    mut ev_render: EventWriter<WorldRenderRequestUpdateEvent>,
) {
    let raycast_source = raycast_source.single();

    // Handle left-click for breaking blocks
    if mouse_input.just_pressed(MouseButton::Left) {
        // Check if there are any intersections with a block
        if let Some((entity, intersection)) = raycast_source.intersections().first() {
            // Check if block is close enough to the player
            if (intersection.position() - p_transform.single_mut().translation).norm()
                < INTERACTION_DISTANCE
            {
                // Remove the hit block
                world_map.remove_block_by_entity(*entity, &mut commands);

                // add the block to the player's inventory
                add_item_to_inventory(&mut player, 1, 1);

                ev_render.send(WorldRenderRequestUpdateEvent());
            }
        }
    }

    // Handle right-click for placing blocks
    if mouse_input.just_pressed(MouseButton::Right) {
        if let Some((_entity, intersection)) = raycast_source.intersections().first() {
            // Get the normal of the face where the block will be placed
            let normal = intersection.normal(); // This is already a Vec3, no need to unwrap
                                                // Calculate the block position by adding a small offset to the intersection point
            let mut position = intersection.position() + normal * 0.51;
            // Snap the position to the grid
            position = snap_to_grid(position);

            // Check if target space is close enough to the player
            // Guarantees a block cannot be placed too close to the player (which would be unable to move because of constant collision)
            if (intersection.position() - p_transform.single_mut().translation).norm()
                <= INTERACTION_DISTANCE
                && (position - p_transform.single_mut().translation).norm() >= CUBE_SIZE
            {
                // Check if the block is in the player's inventory
                if has_item(&mut player, 1) {
                    // Remove the block from the player's inventory
                    remove_item_from_inventory(&mut player, 1, 1);
                } else {
                    return;
                }

                world_map.set_block(
                    &IVec3::new(position.x as i32, position.y as i32, position.z as i32),
                    Block::Dirt,
                );
    
                ev_render.send(WorldRenderRequestUpdateEvent());
            }
        }
    }
}
