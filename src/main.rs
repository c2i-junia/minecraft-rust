use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

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
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 400.0, 
        })
        .add_systems(Startup, setup_world)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, setup_fps_counter)
        .add_systems(Startup, cursor_grab_system)
        .add_systems(Update, player_movement_system)
        .add_systems(Update, camera_control_system)
        .add_systems(Update, (fps_text_update_system, fps_counter_showhide))
        .run();
}
