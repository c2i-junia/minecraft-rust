mod resources;
mod systems;

use bevy::app::ScheduleRunnerPlugin;
use bevy::log::LogPlugin;
use bevy_app::{App, Startup, Update};
use bevy_core::{FrameCountPlugin, TaskPoolPlugin, TypeRegistrationPlugin};
use std::time::Duration;

use crate::systems::{events, init_server};
use naia_bevy_server::{Plugin as ServerPlugin, ServerConfig};

fn main() {
    println!("Naia Bevy Server Demo starting up");

    let mut server_config = ServerConfig::default();
    server_config.connection.disconnection_timeout_duration = Duration::from_secs(10);

    // Build App
    App::default()
        // Plugins
        .add_plugins(TaskPoolPlugin::default())
        .add_plugins(TypeRegistrationPlugin::default())
        .add_plugins(FrameCountPlugin::default())
        // this is needed to avoid running the server at uncapped FPS
        .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_millis(3)))
        .add_plugins(LogPlugin::default())
        .add_plugins(ServerPlugin::new(server_config, shared::protocol()))
        .add_systems(Startup, init_server)
        .add_systems(Update, events::auth_events)
        .run();
}
