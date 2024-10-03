#![allow(clippy::type_complexity)]

mod camera;
mod constants;
mod game;
mod input;
mod lighting;
mod menu;
mod network;
mod player;
mod splash_screen;
mod ui;
mod world;

use bevy::{
    prelude::*,
    render::{
        render_resource::WgpuFeatures,
        settings::{RenderCreation, WgpuSettings},
        RenderPlugin,
    },
};
use menu::settings::{DisplayQuality, Volume};

#[derive(Component)]
pub struct MenuCamera;

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Event)]
pub struct LoadWorldEvent {
    pub world_name: String,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            // Ensures that pixel-art textures will remain pixelated, and not become a blurry mess
            .set(ImagePlugin::default_nearest())
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    // WARN this is a native only feature. It will not work with webgl or webgpu
                    features: WgpuFeatures::POLYGON_MODE_LINE,
                    ..default()
                }),
                ..default()
            }),
    );
    app.add_event::<LoadWorldEvent>();

    server::init("127.0.0.0:0".parse().unwrap());

    network::add_netcode_network(&mut app);
    app.insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .enable_state_scoped_entities::<GameState>()
        // Adds the plugins for each state
        .add_plugins((
            splash_screen::splash_plugin,
            menu::menu_plugin,
            game::game_plugin,
        ))
        .run();
}
