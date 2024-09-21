use bevy::prelude::*;
use camera::*;
use player::*;
use world::*;

mod camera;
mod keyboard;
mod player;
mod world;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup_world)
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, cursor_grab_system)
        .add_systems(Update, player_movement_system)
        .add_systems(Update, camera_control_system)
        .run();
}
