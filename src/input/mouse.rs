use crate::camera::*;
use crate::world::{Block, WORLD_MAP};
use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

// Helper function to snap a Vec3 position to the grid
fn snap_to_grid(position: Vec3) -> Vec3 {
    Vec3::new(position.x.round(), position.y.round(), position.z.round())
}

// Function to handle block placement and breaking
pub fn handle_block_interactions(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>, // to handle mouse input
    mut meshes: ResMut<Assets<Mesh>>,           // for adding new block meshes
    mut materials: ResMut<Assets<StandardMaterial>>, // for adding new block materials
    raycast_source: Query<&RaycastSource<BlockRaycastSet>>, // raycast from the camera
) {
    let dirt_material = materials.add(Color::srgb(0.5, 0.25, 0.0)); // Material for dirt block
    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))); // Cube mesh for the blocks

    let raycast_source = raycast_source.single();

    // Handle left-click for breaking blocks
    if mouse_input.just_pressed(MouseButton::Left) {
        // Check if there are any intersections with a block
        if let Some((entity, _intersection)) = raycast_source.intersections().first() {
            // Remove the hit block
            commands.entity(*entity).despawn();
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

            // Spawn a new dirt block at the snapped position
            commands.spawn((
                PbrBundle {
                    mesh: cube_mesh.clone(),
                    material: dirt_material.clone(),
                    transform: Transform::from_translation(position),
                    ..Default::default()
                },
                RaycastMesh::<BlockRaycastSet>::default(), // Mark the new block as raycastable
            ));
            WORLD_MAP.lock().unwrap().set_block(
                position.x as i32,
                position.y as i32,
                position.z as i32,
                Block::Dirt,
            );
        }
    }
}
