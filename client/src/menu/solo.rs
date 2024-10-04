use std::{fs, path::Path};

use bevy::{
    prelude::{
        BuildChildren, ButtonBundle, Commands, Component, EventWriter, NextState, NodeBundle,
        ResMut, StateScoped, TextBundle,
    },
    text::TextStyle,
    ui::{AlignItems, FlexDirection, JustifyContent, Style, UiRect, Val},
};

use crate::{constants::SAVE_PATH, world::delete_save_files, GameState, LoadWorldEvent};

use super::{MenuButtonAction, MenuState};
use bevy::{
    asset::{AssetServer, Handle},
    color::Color,
    prelude::{Button, Changed, DespawnRecursiveExt, Entity, ImageBundle, Query, Res, With},
    text::{Font, Text, TextSection},
    ui::{
        AlignContent, BackgroundColor, BorderColor, Display, GridPlacement, GridTrack, Interaction,
        UiImage,
    },
    utils::hashbrown::HashMap,
};
use bevy_simple_text_input::{
    TextInputBundle, TextInputInactive, TextInputPlaceholder, TextInputSettings,
    TextInputTextStyle, TextInputValue,
};

pub struct WorldItem {
    pub name: String,
}

#[derive(Component, Default)]
pub struct WorldList {
    pub position: f32,
    pub worlds: HashMap<Entity, WorldItem>,
}

#[derive(Component)]
pub enum MultiplayerButtonAction {
    Add,
    Load(Entity),
    Delete(Entity),
}

#[derive(Component)]
pub struct WorldNameInput;

pub const BACKGROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

pub fn solo_menu_setup(mut commands: Commands, assets: Res<AssetServer>) {
    let font: Handle<Font> = assets.load("fonts/gohu.ttf");
    let txt_style = TextStyle {
        font: font.clone(),
        font_size: 20.,
        color: Color::WHITE,
    };

    let txt_style_inactive = TextStyle {
        font,
        font_size: 20.,
        color: Color::srgb(0.3, 0.3, 0.3),
    };

    let btn_style = Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.)),
        ..Default::default()
    };

    commands
        .spawn((
            StateScoped(MenuState::Solo),
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::horizontal(Val::Percent(20.)),
                    row_gap: Val::Percent(2.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|root| {
            root.spawn(TextBundle {
                text: Text::from_section("World list", txt_style.clone()),
                style: Style {
                    border: UiRect::all(Val::Px(1.)),
                    flex_direction: FlexDirection::Column,
                    align_content: AlignContent::Center,
                    display: Display::Flex,
                    ..Default::default()
                },
                ..Default::default()
            });

            root.spawn((
                NodeBundle {
                    border_color: BorderColor(BACKGROUND_COLOR),
                    style: Style {
                        width: Val::Percent(100.),
                        height: Val::Vh(50.),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Start,
                        justify_content: JustifyContent::Start,
                        border: UiRect::all(Val::Px(2.)),
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                WorldList {
                    position: 0.,
                    worlds: HashMap::new(),
                },
            ));

            root.spawn(NodeBundle {
                style: Style {
                    width: Val::Percent(100.),
                    display: Display::Grid,
                    grid_template_columns: vec![GridTrack::flex(1.), GridTrack::flex(1.)],
                    row_gap: Val::Px(5.),
                    column_gap: Val::Px(5.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|wrapper| {
                wrapper.spawn((
                    NodeBundle {
                        border_color: BorderColor(BACKGROUND_COLOR),
                        background_color: BackgroundColor(Color::BLACK),
                        style: {
                            let mut style = btn_style.clone();
                            style.grid_column = GridPlacement::span(2);
                            style
                        },
                        ..Default::default()
                    },
                    WorldNameInput,
                    TextInputBundle {
                        settings: TextInputSettings {
                            retain_on_submit: true,
                            mask_character: None,
                        },
                        placeholder: TextInputPlaceholder {
                            value: "World name".into(),
                            text_style: Some(txt_style_inactive.clone()),
                        },
                        inactive: TextInputInactive(true),
                        text_style: TextInputTextStyle(txt_style.clone()),
                        ..Default::default()
                    },
                ));

                wrapper
                    .spawn((
                        ButtonBundle {
                            border_color: BorderColor(Color::BLACK),
                            background_color: BackgroundColor(BACKGROUND_COLOR),
                            style: {
                                let mut style = btn_style.clone();
                                style.grid_column = GridPlacement::span(2);
                                style
                            },
                            ..Default::default()
                        },
                        MultiplayerButtonAction::Add,
                    ))
                    .with_children(|btn| {
                        btn.spawn(TextBundle {
                            text: Text::from_section("Create world", txt_style.clone()),
                            ..Default::default()
                        });
                    });

                wrapper
                    .spawn((
                        ButtonBundle {
                            border_color: BorderColor(Color::BLACK),
                            background_color: BackgroundColor(BACKGROUND_COLOR),
                            style: {
                                let mut style = btn_style.clone();
                                style.grid_column = GridPlacement::span(2);
                                style
                            },
                            ..Default::default()
                        },
                        MenuButtonAction::BackToMainMenu,
                    ))
                    .with_children(|btn| {
                        btn.spawn(TextBundle {
                            text: Text::from_section("Back to menu", txt_style.clone()),
                            ..Default::default()
                        });
                    });
            });
        });
}

pub fn list_worlds(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut list_query: Query<(&mut WorldList, Entity)>,
) {
    let (mut list, list_entity) = list_query.single_mut();

    let path = Path::new(SAVE_PATH);

    if !fs::exists(path).unwrap() && fs::create_dir_all(path).is_ok() {
        println!("Successfully created the saves folder : {}", path.display());
    }

    let paths = fs::read_dir(path).unwrap();

    for path in paths {
        let path_str = path.unwrap().file_name().into_string().unwrap();

        if path_str.ends_with("_save.ron") {
            add_world_item(
                path_str.replace("_save.ron", ""),
                &mut commands,
                &assets,
                &mut list,
                list_entity,
            );
        }
    }
}

fn add_world_item(
    name: String,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    list: &mut WorldList,
    entity: Entity,
) {
    println!("Adding world to list : name = {:?}", name);

    let btn_style = Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        border: UiRect::all(Val::Px(2.)),
        height: Val::Percent(80.),
        ..Default::default()
    };

    let img_style = Style {
        height: Val::Percent(100.),
        ..Default::default()
    };

    let world = commands
        .spawn(NodeBundle {
            border_color: BorderColor(BACKGROUND_COLOR),
            style: Style {
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                column_gap: Val::Px(5.),
                width: Val::Percent(100.),
                height: Val::Vh(10.),
                padding: UiRect::horizontal(Val::Percent(2.)),
                border: UiRect::all(Val::Px(2.)),
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let play_btn = commands
        .spawn((
            MultiplayerButtonAction::Load(world),
            ButtonBundle {
                style: btn_style.clone(),
                ..Default::default()
            },
        ))
        .with_children(|btn| {
            let icon = asset_server.load("./play.png");
            btn.spawn(ImageBundle {
                image: UiImage::new(icon),
                style: img_style.clone(),
                ..Default::default()
            });
        })
        .id();

    let delete_btn = commands
        .spawn((
            MultiplayerButtonAction::Delete(world),
            ButtonBundle {
                style: btn_style.clone(),
                ..Default::default()
            },
        ))
        .with_children(|btn| {
            let icon = asset_server.load("./trash.png");
            btn.spawn(ImageBundle {
                image: UiImage::new(icon),
                style: img_style.clone(),
                ..Default::default()
            });
        })
        .id();

    let txt = commands
        .spawn(TextBundle {
            text: Text {
                sections: vec![TextSection {
                    value: name.clone() + "\n",
                    style: TextStyle {
                        font: asset_server.load("fonts/gohu.ttf"),
                        font_size: 20.,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            style: Style {
                display: Display::Flex,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    commands
        .entity(world)
        .push_children(&[play_btn, delete_btn, txt]);

    commands.entity(entity).push_children(&[world]);

    list.worlds.insert(world, WorldItem { name: name.clone() });
}

pub fn solo_action(
    queries: (
        Query<(&Interaction, &MultiplayerButtonAction), (Changed<Interaction>, With<Button>)>,
        Query<&mut TextInputValue, With<WorldNameInput>>,
        Query<(Entity, &mut WorldList), With<WorldList>>,
    ),
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut load_event: EventWriter<LoadWorldEvent>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    let (interaction_query, mut name_query, mut list_query) = queries;
    if list_query.is_empty() {
        return;
    }

    let (entity, mut list) = list_query.single_mut();

    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match *menu_button_action {
                MultiplayerButtonAction::Add => {
                    if !name_query.is_empty() {
                        let mut name = name_query.single_mut();

                        add_world_item(
                            name.0.clone(),
                            &mut commands,
                            &asset_server,
                            &mut list,
                            entity,
                        );

                        name.0 = "".into();
                    }
                }
                MultiplayerButtonAction::Load(world_entity) => {
                    if let Some(world) = list.worlds.get(&world_entity) {
                        println!("World : name={}, {}", world.name, list.position);

                        load_event.send(LoadWorldEvent {
                            world_name: world.name.clone(),
                        });
                        game_state.set(GameState::PreGameLoading);
                        menu_state.set(MenuState::Disabled);
                    }
                }
                MultiplayerButtonAction::Delete(world_entity) => {
                    if let Some(world) = list.worlds.get(&world_entity) {
                        if let Err(e) = delete_save_files(&world.name) {
                            println!("Error while deleting save files: {}", e);
                        }
                        list.worlds.remove(&world_entity);
                    }
                    commands.entity(entity).remove_children(&[world_entity]);
                    commands.entity(world_entity).despawn_recursive();
                }
            }
        }
    }
}
