use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;

use bevy::{app::AppExit, color::palettes::css::CRIMSON};
use bevy_simple_text_input::TextInputInactive;
use controls::{controls_menu_setup, controls_update_system};
use multi::multiplayer_action;
use shared::world::ItemBlockRegistry;

use crate::input::keyboard::save_keybindings;
use crate::{DisplayQuality, GameState, MenuCamera, Volume, TEXT_COLOR};

pub mod controls;
pub mod game_loading_screen;
pub mod multi;
pub mod settings;
pub mod solo;

// This plugin manages the menu, with 5 different screens:
// - a main menu with "New Game", "Settings", "Quit"
// - a settings menu with two submenus and a back button
// - two settings screen with a setting that can be set and a back button
pub fn menu_plugin(app: &mut App) {
    app
        // At start, the menu is not enabled. This will be changed in `menu_setup` when
        // entering the `GameState::Menu` state.
        // Current screen in the menu is handled by an independent state from `GameState`
        .init_state::<MenuState>()
        .enable_state_scoped_entities::<MenuState>()
        .add_systems(OnEnter(GameState::Menu), menu_setup)
        // Systems to handle the main menu screen
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        // Systems to handle the play menu screen
        .add_systems(
            OnEnter(MenuState::Solo),
            (solo::solo_menu_setup, solo::list_worlds).chain(),
        )
        .add_systems(Update, solo::solo_action.run_if(in_state(MenuState::Solo)))
        // Systems to handle the settings menu screen
        .add_systems(OnEnter(MenuState::Settings), settings::settings_menu_setup)
        // Systems to handle the display settings screen
        .add_systems(
            OnEnter(MenuState::SettingsDisplay),
            settings::display_settings_menu_setup,
        )
        .add_systems(
            Update,
            (settings::setting_button::<DisplayQuality>
                .run_if(in_state(MenuState::SettingsDisplay)),),
        )
        // Systems to handle the sound settings screen
        .add_systems(
            OnEnter(MenuState::SettingsSound),
            settings::sound_settings_menu_setup,
        )
        .add_systems(OnExit(MenuState::SettingsControls), save_keybindings)
        .add_systems(
            Update,
            settings::setting_button::<Volume>.run_if(in_state(MenuState::SettingsSound)),
        )
        .add_systems(OnEnter(MenuState::Multi), multi::multiplayer_menu_setup)
        .add_systems(
            Update,
            (multiplayer_action).run_if(in_state(MenuState::Multi)),
        )
        .add_systems(
            Update,
            controls_update_system.run_if(in_state(MenuState::SettingsControls)),
        )
        // Common systems to all screens that handles buttons behavior
        .add_systems(
            Update,
            (menu_action, button_system, mouse_scroll).run_if(in_state(GameState::Menu)),
        )
        .add_systems(OnEnter(MenuState::SettingsControls), controls_menu_setup);
}

// State used for the current menu screen
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Solo,
    Multi,
    Settings,
    SettingsDisplay,
    SettingsSound,
    SettingsControls,
    #[default]
    Disabled,
}

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const HOVERED_PRESSED_BUTTON: Color = Color::srgb(0.25, 0.65, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

/// Tag component for scrolling UI lists
#[derive(Component)]
pub struct ScrollingList {
    pub position: f32,
}

// Tag component used to mark which setting is currently selected
#[derive(Component)]
pub struct SelectedOption;

// All actions that can be triggered from a button click
#[derive(Component)]
enum MenuButtonAction {
    Solo,
    Multi,
    Settings,
    SettingsDisplay,
    SettingsSound,
    SettingsControls,
    BackToMainMenu,
    BackToSettings,
    Quit,
}

// This system handles changing all buttons color based on mouse interaction
fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
    input_click_query: Query<
        (Entity, &Interaction),
        (Changed<Interaction>, With<TextInputInactive>),
    >,
    mut text_input_query: Query<(Entity, &mut TextInputInactive)>,
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }

    for (interaction_entity, interaction) in &input_click_query {
        if *interaction == Interaction::Pressed {
            for (entity, mut inactive) in &mut text_input_query {
                inactive.0 = entity != interaction_entity;
            }
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>, mut commands: Commands, registry: Res<ItemBlockRegistry>) {
    println!("Registry : {:?}", registry);
    commands.spawn((
        Camera2dBundle::default(),
        MenuCamera,
        StateScoped(GameState::Menu),
    ));
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Common style for all buttons on the screen
    let button_style = Style {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_icon_style = Style {
        width: Val::Px(30.0),
        // This takes the icons out of the flexbox flow, to be positioned exactly
        position_type: PositionType::Absolute,
        // The icon will be close to the left border of the button
        left: Val::Px(10.0),
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font_size: 33.0,
        color: TEXT_COLOR,
        ..Default::default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            StateScoped(MenuState::Main),
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    background_color: CRIMSON.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "minecraft-rust",
                            TextStyle {
                                font_size: 67.0,
                                color: TEXT_COLOR,
                                ..Default::default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..Default::default()
                        }),
                    );

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - quit
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..Default::default()
                            },
                            MenuButtonAction::Solo,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("./right.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..Default::default()
                            });
                            parent
                                .spawn(TextBundle::from_section("Solo", button_text_style.clone()));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..Default::default()
                            },
                            MenuButtonAction::Multi,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("./multi.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..Default::default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Multi",
                                button_text_style.clone(),
                            ));
                        });

                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style.clone(),
                                background_color: NORMAL_BUTTON.into(),
                                ..Default::default()
                            },
                            MenuButtonAction::Settings,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("./wrench.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style.clone(),
                                image: UiImage::new(icon),
                                ..Default::default()
                            });
                            parent.spawn(TextBundle::from_section(
                                "Settings",
                                button_text_style.clone(),
                            ));
                        });
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..Default::default()
                            },
                            MenuButtonAction::Quit,
                        ))
                        .with_children(|parent| {
                            let icon = asset_server.load("./exitRight.png");
                            parent.spawn(ImageBundle {
                                style: button_icon_style,
                                image: UiImage::new(icon),
                                ..Default::default()
                            });
                            parent.spawn(TextBundle::from_section("Quit", button_text_style));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.send(AppExit::Success);
                }
                MenuButtonAction::Solo => menu_state.set(MenuState::Solo),
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsSound);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
                MenuButtonAction::Multi => menu_state.set(MenuState::Multi),
                MenuButtonAction::SettingsControls => menu_state.set(MenuState::SettingsControls),
            }
        }
    }
}

pub fn mouse_scroll(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut query_list: Query<(&mut ScrollingList, &mut Style, &Parent, &Node)>,
    query_node: Query<&Node>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        println!("MouseEvent");
        for (mut scrolling_list, mut style, parent, list_node) in &mut query_list {
            let items_height = list_node.size().y;
            let container_height = query_node.get(parent.get()).unwrap().size().y;

            let max_scroll = (items_height - container_height).max(0.) + 30.;

            let dy = match mouse_wheel_event.unit {
                MouseScrollUnit::Line => mouse_wheel_event.y * 20.,
                MouseScrollUnit::Pixel => mouse_wheel_event.y,
            };

            scrolling_list.position += dy;
            scrolling_list.position = scrolling_list.position.clamp(-max_scroll, 0.);
            style.top = Val::Px(scrolling_list.position);
            println!(
                "Mouse event : {:?}, {:?}, {:?}, {:?}",
                container_height, max_scroll, items_height, scrolling_list.position
            );
        }
    }
}
