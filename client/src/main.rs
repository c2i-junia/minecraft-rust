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
use input::keyboard::GameAction;
use menu::settings::{DisplayQuality, Volume};
use std::collections::HashMap;

#[derive(Component)]
pub struct MenuCamera;

pub const TEXT_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Splash,
    Menu,
    PreGameLoading,
    Game,
}

#[derive(Event)]
pub struct LoadWorldEvent {
    pub world_name: String,
}

#[derive(Resource)]
pub struct KeyMap {
    pub map: HashMap<GameAction, Vec<KeyCode>>
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
    network::add_base_netcode(&mut app);
    app.insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .insert_resource(KeyMap {
            map: {
                let mut map = HashMap::new();
                map.insert(
                    GameAction::MoveForward,
                    vec![KeyCode::KeyW, KeyCode::ArrowUp],
                );
                map.insert(
                    GameAction::MoveBackward,
                    vec![KeyCode::KeyS, KeyCode::ArrowDown],
                );
                map.insert(
                    GameAction::MoveLeft,
                    vec![KeyCode::KeyA, KeyCode::ArrowLeft],
                );
                map.insert(
                    GameAction::MoveRight,
                    vec![KeyCode::KeyD, KeyCode::ArrowRight],
                );
                map.insert(GameAction::Jump, vec![KeyCode::Space]);
                map.insert(GameAction::Escape, vec![KeyCode::Escape]);
                map.insert(GameAction::ToggleFps, vec![KeyCode::F3]);
                map.insert(GameAction::ToggleViewMode, vec![KeyCode::F5]);
                map.insert(GameAction::ToggleChunkDebugMode, vec![KeyCode::F4]);
                map.insert(GameAction::ToggleFlyMode, vec![KeyCode::KeyF]);
                map.insert(GameAction::FlyUp, vec![KeyCode::Space]);
                map.insert(GameAction::FlyDown, vec![KeyCode::ShiftLeft]);
                map.insert(GameAction::ToggleBlockWireframeDebugMode, vec![KeyCode::F6]);
                map.insert(GameAction::ToggleInventory, vec![KeyCode::KeyE]);
                map.insert(GameAction::OpenChat, vec![KeyCode::KeyT]);
                map.insert(GameAction::RenderDistanceMinus, vec![KeyCode::KeyO]);
                map.insert(GameAction::RenderDistancePlus, vec![KeyCode::KeyP]);
                map
            }
        })
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
