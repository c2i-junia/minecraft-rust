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
        .add_startup_system(setup_world)
        .add_startup_system(cursor_grab_system)
        .add_system(player_movement_system)
        .add_system(camera_control_system)
        .run();
}
