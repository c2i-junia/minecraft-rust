use bevy::color::palettes::basic::WHITE;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::render_resource::WgpuFeatures;
use bevy::render::settings::{RenderCreation, WgpuSettings};
use bevy::render::RenderPlugin;
use bevy_mod_raycast::deferred::DeferredRaycastingPlugin;
use block::block_text_update_system;
use lighting::setup_main_lighting;

use crate::chunk_debug_ghost::{chunk_ghost_update_system, setup_chunk_ghost};
use block_debug_wireframe::*;
use camera::*;
use exit::*;
use hud::*;
use input::*;
use lighting::*;
use materials::*;
use player::*;
use ui::{inventory::*, set_ui_mode};
use world::*;

mod block_debug_wireframe;
mod camera;
mod chunk_debug_ghost;
mod constants;
mod exit;
mod hud;
mod input;
mod items;
mod lighting;
mod materials;
mod player;
mod ui;
mod utils;
mod world;

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
        .add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(DeferredRaycastingPlugin::<BlockRaycastSet>::default()) // Ajout du plugin raycasting
        .add_plugins(WireframePlugin)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 400.0,
        })
        .insert_resource(WorldMap { ..default() })
        .insert_resource(BlockDebugWireframeSettings { is_enabled: false })
        .insert_resource(WireframeConfig {
            // The global wireframe config enables drawing of wireframes on every mesh,
            // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
            // regardless of the global configuration.
            global: false,
            // Controls the default color of all wireframes. Used as the default color for global wireframes.
            // Can be changed per mesh using the `WireframeColor` component.
            default_color: WHITE.into(),
        })
        .insert_resource(MaterialResource { ..default() })
        .insert_resource(AtlasHandles { ..default() })
        .add_event::<WorldRenderRequestUpdateEvent>()
        .add_systems(
            Startup,
            (
                setup_materials,
                setup_world,
                spawn_player,
                setup_main_lighting,
            )
                .chain(),
        )
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_reticle)
        .add_systems(Startup, setup_hud)
        .add_systems(Startup, setup_inventory)
        .add_systems(Startup, cursor_grab_system)
        .add_systems(Startup, setup_chunk_ghost)
        .add_systems(Update, toggle_inventory)
        .add_systems(Update, set_ui_mode)
        .add_systems(Update, build_atlas)
        .add_systems(Update, player_movement_system)
        .add_systems(
            Update,
            (handle_block_interactions, camera_control_system).chain(),
        )
        .add_systems(Update, fps_text_update_system)
        .add_systems(Update, inventory_update_system)
        .add_systems(Update, coords_text_update_system)
        .add_systems(Update, total_blocks_text_update_system)
        .add_systems(Update, block_text_update_system)
        .add_systems(Update, toggle_hud_system)
        .add_systems(Update, chunk_ghost_update_system)
        .add_systems(Update, exit_system)
        .add_systems(Update, toggle_wireframe_system)
        .add_systems(Update, world_render_system)
        .add_systems(Update, set_cursor_visibility)
        .add_systems(Update, inventory_cell_interaction_system)
        .add_systems(Update, update_celestial_bodies)
        .run();
}
