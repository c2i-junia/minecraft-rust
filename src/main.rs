use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy_mod_raycast::deferred::DeferredRaycastingPlugin;

use crate::chunk_debug_ghost::{chunk_ghost_update_system, setup_chunk_ghost};
use block_debug_wireframe::*;
use camera::*;
use exit::*;
use hud::*;
use input::*;
use materials::*;
use player::*;
use world::*;

mod block_debug_wireframe;
mod camera;
mod chunk_debug_ghost;
mod constants;
mod exit;
mod hud;
mod input;
mod materials;
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
        .insert_resource(WorldMap { ..default() })
        .insert_resource(BlockDebugWireframeSettings { is_enabled: false })
        .add_systems(
            Startup,
            (setup_materials, setup_cube_mesh, setup_world).chain(),
        )
        .add_systems(Startup, spawn_player)
        .add_systems(Startup, spawn_camera)
        .add_systems(Startup, spawn_reticle)
        .add_systems(Startup, setup_ui)
        .add_systems(Startup, cursor_grab_system)
        .add_systems(Startup, setup_chunk_ghost)
        .add_systems(Update, player_movement_system)
        .add_systems(Update, camera_control_system)
        .add_systems(Update, fps_text_update_system)
        .add_systems(Update, inventory_text_update_system)
        .add_systems(Update, coords_text_update_system)
        .add_systems(Update, total_blocks_text_update_system)
        .add_systems(Update, toggle_hud_system)
        .add_systems(Update, handle_block_interactions) // Ajout du système de clic pour casser les blocs
        .add_systems(Update, chunk_ghost_update_system)
        .add_systems(Update, exit_system)
        .add_systems(Update, toggle_wireframe_system)
        .run();
}
