mod camera;
mod constants;
mod exit;
mod hud;
mod input;
mod lighting;
mod network;
mod player;
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

use crate::debug::BlockDebugWireframeSettings;
use bevy::color::palettes::basic::WHITE;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::pbr::wireframe::{WireframeConfig, WireframePlugin};

use crate::hud::debug::targeted_block::block_text_update_system;
use crate::lighting::setup_main_lighting;
use bevy_mod_raycast::deferred::DeferredRaycastingPlugin;
use naia_bevy_client::{ClientConfig, Plugin as ClientPlugin};

use crate::exit::*;
use crate::hud::debug::*;
use crate::hud::hotbar::*;
use crate::lighting::*;
use crate::network::*;
use crate::ui::set_ui_mode;
use crate::world::*;

// Enum that will be used as a global state for the game
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Game,
}

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
        // Declare the game state, whose starting value is determined by the `Default` trait
        .init_state::<GameState>()
        // Adds the plugins for each state
        .add_plugins((splash::splash_plugin, game::game_plugin))
        .run();
}

mod splash {
    use bevy::prelude::*;

    use super::{despawn_screen, GameState};

    // This plugin will display a splash screen with Bevy logo for 1 second before switching to the menu
    pub fn splash_plugin(app: &mut App) {
        // As this plugin is managing the splash screen, it will focus on the state `GameState::Splash`
        app
            // When entering the state, spawn everything needed for this screen
            .add_systems(OnEnter(GameState::Splash), splash_setup)
            // While in this state, run the `countdown` system
            .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
            // When exiting the state, despawn everything that was spawned for this screen
            .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
    }

    // Tag component used to tag entities added on the splash screen
    #[derive(Component)]
    struct OnSplashScreen;

    // Newtype to use a `Timer` for this screen as a resource
    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Display the logo
        let icon = asset_server.load("./bevy_icon.png");
        println!("splash screen");

        // create 2d camera for spash screen
        commands.spawn((Camera2dBundle::default(), OnSplashScreen));

        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                OnSplashScreen,
            ))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: Style {
                        // This will set the logo to be 200px wide, and auto adjust its height
                        width: Val::Px(200.0),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                });
            });
        // Insert the timer as a resource
        commands.insert_resource(SplashTimer(Timer::from_seconds(1.0, TimerMode::Once)));
    }

    // Tick the timer, and change state when finished
    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Game);
        }
    }
}

mod game {
    use super::*;

    // This plugin will contain the game. In this case, it's just be a screen that will
    // display the current settings for 5 seconds before returning to the menu
    pub fn game_plugin(app: &mut App) {
        app.add_plugins(ClientPlugin::<network::MainClient>::new(
            ClientConfig::default(),
            shared::protocol(),
        ))
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
            OnEnter(GameState::Game),
            (
                setup_materials,
                setup_world,
                spawn_player,
                setup_main_lighting,
            )
                .chain(),
        )
        .add_systems(OnEnter(GameState::Game), spawn_camera)
        .add_systems(OnEnter(GameState::Game), spawn_reticle)
        .add_systems(OnEnter(GameState::Game), setup_hud)
        .add_systems(
            OnEnter(GameState::Game),
            (setup_hotbar, setup_inventory).chain(),
        )
        .add_systems(OnEnter(GameState::Game), mouse_grab_system)
        .add_systems(OnEnter(GameState::Game), setup_chunk_ghost)
        .add_systems(OnEnter(GameState::Game), init_network_socket)
        .add_systems(
            Update,
            (
                toggle_inventory,
                set_ui_mode,
                build_atlas,
                player_movement_system,
                (handle_block_interactions, camera_control_system).chain(),
                fps_text_update_system,
                inventory_update_system,
                coords_text_update_system,
                total_blocks_text_update_system,
                block_text_update_system,
                toggle_hud_system,
                chunk_ghost_update_system,
                exit_system,
                toggle_wireframe_system,
                world_render_system,
                set_mouse_visibility,
                inventory_cell_interaction_system,
                update_celestial_bodies,
            )
                .run_if(in_state(GameState::Game)),
        );
    }
}

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
