use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use crate::camera::*;

pub fn handle_block_breaking(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,  // to handle mouse input
    raycast_source: Query<&RaycastSource<BlockRaycastSet>>,  // raycast from the camera
) {
    // check if the left mouse button was pressed
    if mouse_input.just_pressed(MouseButton::Left) {
        let raycast_source = raycast_source.single();

        // check if there are any intersections with a block
        if let Some((entity, _intersection)) = raycast_source.intersections().first() {
            // println!("block hit, removing...");
            // remove the hit block
            commands.entity(*entity).despawn();
        }
    }
}
