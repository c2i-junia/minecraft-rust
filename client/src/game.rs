use bevy::app::MainScheduleOrder;
use bevy::ecs::schedule::ScheduleLabel;
use bevy::prelude::*;
use chat::{render_chat, setup_chat};

use crate::debug::BlockDebugWireframeSettings;
use crate::ui::pause::{render_pause_menu, setup_pause_menu};
use bevy::color::palettes::basic::WHITE;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};

use crate::hud::debug::targeted_block::block_text_update_system;
use crate::lighting::setup_main_lighting;
use bevy_mod_raycast::deferred::DeferredRaycastingPlugin;

use crate::hud::debug::*;
use crate::hud::hotbar::*;
use crate::lighting::*;
use crate::ui::set_ui_mode;
use crate::world::*;

use crate::camera::*;
use crate::hud::*;
use crate::input::*;
use crate::player::*;
use crate::ui::inventory::*;

use crate::{despawn_menu_camera, DisplayQuality, GameState, Volume};

fn print_settings(display_quality: Res<DisplayQuality>, volume: Res<Volume>) {
    println!("Entering GameState::Game");
    println!("Current Display Quality: {:?}", *display_quality);
    println!("Current Volume: {:?}", *volume);
}

#[derive(ScheduleLabel, Debug, Clone, PartialEq, Eq, Hash)]
struct UiUpdate;

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub fn game_plugin(app: &mut App) {
    // Adds the `UiUpdate` schedule, to run before the Update
    app.init_schedule(UiUpdate);

    app.world_mut()
        .resource_mut::<MainScheduleOrder>()
        .insert_after(StateTransition, UiUpdate);

    app.add_plugins(FrameTimeDiagnosticsPlugin)
        .add_plugins(DeferredRaycastingPlugin::<BlockRaycastSet>::default()) // Ajout du plugin raycasting
        .add_plugins(WireframePlugin)
        .add_plugins(bevy_simple_text_input::TextInputPlugin)
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
        .add_event::<WorldRenderRequestUpdateEvent>()
        .add_systems(
            OnEnter(GameState::Game),
            (
                despawn_menu_camera,
                setup_materials,
                setup_world,
                spawn_player,
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
        .add_systems(OnEnter(GameState::Game), print_settings)
        .add_systems(OnEnter(GameState::Game), mouse_grab_system)
        .add_systems(OnEnter(GameState::Game), setup_chunk_ghost)
        .add_systems(
            UiUpdate,
            (
                render_pause_menu,
                render_chat,
                render_inventory_hotbar,
                set_ui_mode,
            )
                .chain()
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
        .add_systems(PostUpdate, (world_render_system,));
}
