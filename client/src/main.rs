mod camera;
mod constants;
mod exit;
mod game;
mod hud;
mod input;
mod lighting;
mod menu;
mod network;
mod player;
mod splash_screen;
mod ui;
mod world;

use bevy::prelude::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;

use camera::*;
use hud::*;
use input::*;
use player::*;
use ui::inventory::*;

use crate::world::*;

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

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub enum DisplayQuality {
    Low,
    Medium,
    High,
}

// One of the two settings that can be set through the menu. It will be a resource in the app
#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Volume(u32);

fn main() {
    App::new()
        .add_plugins(
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
        )
        // .add_plugins(DefaultPlugins)
        // Insert as resource_equalsource the initial value for the settings resources
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        // Adds the plugins for each state
        .add_plugins((
            splash_screen::splash_plugin,
            menu::menu_plugin,
            game::game_plugin,
        ))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MenuCamera));
}

fn despawn_menu_camera(
    mut commands: Commands,
    query: Query<Entity, With<MenuCamera>>, // Filtre pour les entités marquées avec `MenuCamera`
) {
    for entity in &query {
        commands.entity(entity).despawn_recursive(); // Supprimer l'entité et tous ses enfants
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
