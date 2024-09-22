use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use bevy_mod_raycast::deferred::DeferredRaycastingPlugin;

use camera::*;
use fps_counter::*;
use player::*;
use world::*;

mod camera;
mod fps_counter;
mod keyboard;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(DeferredRaycastingPlugin::<BlockRaycastSet>::default()) // Ajout du plugin raycasting
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 400.0,
        })
        .add_systems(Startup, setup_world)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_reticle)
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Startup, cursor_grab_system)
        .add_systems(Update, player_movement_system)
        .add_systems(Update, camera_control_system)
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .add_systems(Update, handle_block_breaking) // Ajout du système de clic pour casser les blocs
        .run();
}

fn handle_block_breaking(
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

