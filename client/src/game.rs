use std::collections::HashMap;

use crate::ui::chat::{render_chat, setup_chat};
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;

use crate::ui::debug::BlockDebugWireframeSettings;
use crate::ui::pause::{render_pause_menu, setup_pause_menu};
use crate::ui::reticle::spawn_reticle;
use bevy::color::palettes::basic::WHITE;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};

use crate::lighting::setup_main_lighting;
use crate::ui::debug::targeted_block::block_text_update_system;
use bevy_mod_raycast::deferred::DeferredRaycastingPlugin;

use crate::lighting::*;
use crate::ui::debug::*;
use crate::ui::hotbar::*;
use crate::ui::set_ui_mode;
use crate::world::*;

use crate::camera::*;
use crate::input::*;
use crate::player::*;
use crate::ui::inventory::*;

use crate::menu::game_loading_screen::load_loading_screen;
use crate::network::{
    establish_authenticated_connection_to_server, init_server_connection,
    launch_local_server_system, network_failure_handler, poll_network_messages,
    terminate_server_connection, upload_player_inputs_system,
};
use crate::{DisplayQuality, GameState, Volume};

fn print_settings(display_quality: Res<DisplayQuality>, volume: Res<Volume>) {
    println!("Entering GameState::Game");
    println!("Current Display Quality: {:?}", *display_quality);
    println!("Current Volume: {:?}", *volume);
}

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub fn game_plugin(app: &mut App) {
    app.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(DeferredRaycastingPlugin::<BlockRaycastSet>::default()) // Ajout du plugin raycasting
        .add_plugins(WireframePlugin)
        .add_plugins(bevy_simple_text_input::TextInputPlugin)
        .add_plugins(AtmospherePlugin)
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
        .insert_resource(RenderDistance { ..default() })
        .insert_resource(UIMode::Closed)
        .add_event::<WorldRenderRequestUpdateEvent>()
        .add_event::<SaveRequestEvent>()
        .add_systems(
            OnEnter(GameState::PreGameLoading),
            (
                load_loading_screen,
                launch_local_server_system,
                init_server_connection,
            )
                .chain(),
        )
        .add_systems(
            Update,
            establish_authenticated_connection_to_server
                .run_if(in_state(GameState::PreGameLoading)),
        )
        .add_systems(
            OnEnter(GameState::Game),
            (
                setup_materials,
                spawn_player,
                setup_world,
                setup_main_lighting,
                spawn_camera,
                spawn_reticle,
                setup_hud,
                setup_chat,
                setup_pause_menu,
            )
                .chain(),
        )
        .add_systems(
            OnEnter(GameState::Game),
            (setup_hotbar, setup_inventory).chain(),
        )
        .add_systems(
            OnEnter(GameState::Game),
            (print_settings, setup_chunk_ghost),
        )
        .add_systems(
            Update,
            (
                render_pause_menu,
                render_chat,
                render_inventory_hotbar,
                set_ui_mode,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (
                build_atlas,
                player_movement_system,
                (handle_block_interactions, camera_control_system).chain(),
                fps_text_update_system,
                coords_text_update_system,
                total_blocks_text_update_system,
                block_text_update_system,
                toggle_hud_system,
                chunk_ghost_update_system,
                toggle_wireframe_system,
                set_mouse_visibility,
                update_celestial_bodies,
                render_distance_update_system,
            )
                .chain()
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(Update, save_world_system.run_if(in_state(GameState::Game)))
        .add_systems(
            PostUpdate,
            (world_render_system,).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            Update,
            (
                poll_network_messages,
                network_failure_handler,
                upload_player_inputs_system,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnExit(GameState::Game),
            (clear_resources, terminate_server_connection).chain(),
        );
}

fn clear_resources(mut world_map: ResMut<WorldMap>) {
    world_map.map = HashMap::new();
    world_map.total_blocks_count = 0;
    world_map.total_chunks_count = 0;
    world_map.name = "".into();
}
