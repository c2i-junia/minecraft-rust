use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_mod_raycast::deferred::DeferredRaycastingPlugin;

use camera::*;
use fps_counter::*;
use input::*;
use player::*;
use world::*;

mod camera;
mod fps_counter;
mod input;
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
