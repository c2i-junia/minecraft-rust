use std::{
    fs,
    io::Write,
    path::Path,
};

use bevy::{
    asset::{AssetServer, Handle},
    color::Color,
    prelude::{
        BuildChildren, Button, ButtonBundle, Changed, Commands, Component, DespawnRecursiveExt,
        Entity, ImageBundle, NodeBundle, Query, Res, StateScoped, TextBundle, With, Without,
    },
    text::{Font, Text, TextSection, TextStyle},
    ui::{
        AlignContent, AlignItems, BackgroundColor, BorderColor, Display, FlexDirection,
        GridPlacement, GridTrack, Interaction, JustifyContent, Overflow, Style, UiImage, UiRect,
        Val,
    },
    utils::hashbrown::HashMap,
};
use bevy_simple_text_input::{
    TextInputBundle, TextInputInactive, TextInputPlaceholder, TextInputSettings,
    TextInputTextStyle, TextInputValue,
};
use ron::{from_str, ser::PrettyConfig};

use crate::constants::{SAVE_PATH, SERVER_LIST_SAVE_NAME};

use super::{MenuButtonAction, MenuState, ScrollingList};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct ServerItem {
    pub name: String,
    pub ip: String,
}

#[derive(Component, Default)]
pub struct ServerList {
    pub servers: HashMap<Entity, ServerItem>,
}

#[derive(Component)]
pub enum MultiplayerButtonAction {
    Add,
    Connect(Entity),
    Delete(Entity),
}

#[derive(Component)]
pub struct ServerIpInput;

#[derive(Component)]
pub struct ServerNameInput;

pub const BACKGROUND_COLOR: Color = Color::srgb(0.5, 0.5, 0.5);

pub fn multiplayer_menu_setup(mut commands: Commands, assets: Res<AssetServer>) {
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
            StateScoped(MenuState::Multi),
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
                text: Text::from_section("Server list", txt_style.clone()),
                style: Style {
                    border: UiRect::all(Val::Px(1.)),
                    flex_direction: FlexDirection::Column,
                    align_content: AlignContent::Center,
                    display: Display::Flex,
                    ..Default::default()
                },
                ..Default::default()
            });

            root.spawn(NodeBundle {
                border_color: BorderColor(BACKGROUND_COLOR),
                style: Style {
                    width: Val::Percent(100.),
                    height: Val::Percent(50.),
                    flex_direction: FlexDirection::Column,
                    overflow: Overflow::clip_y(),
                    border: UiRect::all(Val::Px(2.)),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|w| {
                w.spawn((
                    NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(Val::Px(10.)),
                            row_gap: Val::Px(10.),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ScrollingList { position: 0. },
                    ServerList {
                        servers: HashMap::new(),
                    },
                ));
            });

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
                    ServerNameInput,
                    TextInputBundle {
                        settings: TextInputSettings {
                            retain_on_submit: true,
                            mask_character: None,
                        },
                        placeholder: TextInputPlaceholder {
                            value: "Server name".into(),
                            text_style: Some(txt_style_inactive.clone()),
                        },
                        inactive: TextInputInactive(true),
                        text_style: TextInputTextStyle(txt_style.clone()),
                        ..Default::default()
                    },
                ));

                wrapper.spawn((
                    NodeBundle {
                        border_color: BorderColor(BACKGROUND_COLOR),
                        background_color: BackgroundColor(Color::BLACK),
                        style: btn_style.clone(),
                        ..Default::default()
                    },
                    TextInputBundle {
                        settings: TextInputSettings {
                            retain_on_submit: true,
                            mask_character: None,
                        },
                        placeholder: TextInputPlaceholder {
                            value: "Server IP".into(),
                            text_style: Some(txt_style_inactive.clone()),
                        },
                        inactive: TextInputInactive(true),
                        text_style: TextInputTextStyle(txt_style.clone()),
                        ..Default::default()
                    },
                    ServerIpInput,
                ));

                wrapper
                    .spawn((
                        ButtonBundle {
                            border_color: BorderColor(Color::BLACK),
                            background_color: BackgroundColor(BACKGROUND_COLOR),
                            style: btn_style.clone(),
                            ..Default::default()
                        },
                        MultiplayerButtonAction::Add,
                    ))
                    .with_children(|btn| {
                        btn.spawn(TextBundle {
                            text: Text::from_section("Add server", txt_style.clone()),
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

pub fn add_server_item(
    name: String,
    ip: String,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    list: &mut ServerList,
    list_entity: Entity,
) {
    println!("Adding server to list : name = {:?}, ip = {:?}", name, ip);

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

    let server = commands
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
            MultiplayerButtonAction::Connect(server),
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
            MultiplayerButtonAction::Delete(server),
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
                sections: vec![
                    TextSection {
                        value: name.clone() + "\n",
                        style: TextStyle {
                            font: asset_server.load("fonts/gohu.ttf"),
                            font_size: 20.,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: ip.clone(),
                        style: TextStyle {
                            font: asset_server.load("fonts/gohu.ttf"),
                            font_size: 15.,
                            color: Color::srgb(0.4, 0.4, 0.4),
                        },
                    },
                ],
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
        .entity(server)
        .push_children(&[play_btn, delete_btn, txt]);

    commands.entity(list_entity).push_children(&[server]);

    list.servers.insert(
        server,
        ServerItem {
            name: name.clone(),
            ip: ip.clone(),
        },
    );
}

pub fn load_server_list(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut list_query: Query<(&mut ServerList, Entity)>,
) {
    let (mut list, list_entity) = list_query.single_mut();

    let txt = format!("{}{}", SAVE_PATH, SERVER_LIST_SAVE_NAME);

    let path = Path::new(&txt);

    // If no server list save, returns
    if !fs::exists(path).unwrap() {
        return;
    }

    let txt = fs::read_to_string(path);
    if txt.is_err() {
        return;
    }
    let txt = txt.unwrap();

    let servers = from_str::<Vec<ServerItem>>(&txt);
    if servers.is_err() {
        return;
    }
    let servers = servers.unwrap();

    for srv in servers {
        add_server_item(
            srv.name,
            srv.ip,
            &mut commands,
            &assets,
            &mut list,
            list_entity,
        );
    }
}

pub fn save_server_list(list: Query<&ServerList>) {
    let list = list.single();

    let path = Path::new(SAVE_PATH);

    if !fs::exists(path).unwrap() {
        if fs::create_dir_all(path).is_ok() {
            println!("Successfully created the saves folder : {}", path.display());
        } else {
            println!("Could not create the saves folder, server list save aborted");
            return;
        }
    }

    let pretty_config = PrettyConfig::new()
        .with_depth_limit(3)
        .with_separate_tuple_members(true)
        .with_enumerate_arrays(true);

    if let Ok(data) = ron::ser::to_string_pretty(
        &list
            .servers
            .values()
            .map(|v| (*v).clone())
            .collect::<Vec<ServerItem>>(),
        pretty_config,
    ) {
        if let Ok(mut file) = fs::File::create(Path::new(&format!(
            "{}{}",
            SAVE_PATH, SERVER_LIST_SAVE_NAME
        ))) {
            if file.write_all(data.as_bytes()).is_ok() {
                println!("Server list saved")
            }
        }
    }
}

pub fn multiplayer_action(
    queries: (
        Query<(&Interaction, &MultiplayerButtonAction), (Changed<Interaction>, With<Button>)>,
        Query<&TextInputValue, (With<ServerNameInput>, Without<ServerIpInput>)>,
        Query<&TextInputValue, (With<ServerIpInput>, Without<ServerNameInput>)>,
        Query<(Entity, &mut ServerList), With<ServerList>>,
    ),
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (interaction_query, name_query, ip_query, mut list_query) = queries;
    if list_query.is_empty() {
        return;
    }

    let (entity, mut list) = list_query.single_mut();

    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match *menu_button_action {
                MultiplayerButtonAction::Add => {
                    if !name_query.is_empty() && !ip_query.is_empty() {
                        let name = name_query.single();
                        let ip = ip_query.single();

                        add_server_item(
                            name.0.clone(),
                            ip.0.clone(),
                            &mut commands,
                            &asset_server,
                            &mut list,
                            entity,
                        );
                    }
                }
                MultiplayerButtonAction::Connect(serv_entity) => {
                    if let Some(srv) = list.servers.get(&serv_entity) {
                        println!("Server : name={}, ip={}", srv.name, srv.ip);

                        // TODO : try to connect player with srv.ip provided
                    }
                }
                MultiplayerButtonAction::Delete(serv_entity) => {
                    println!("Old list : {:?}",list.servers);
                    commands.entity(entity).remove_children(&[serv_entity]);
                    commands.entity(serv_entity).despawn_recursive();
                    list.servers.remove(&serv_entity);
                    println!("New list : {:?}",list.servers);
                }
            }
        }
    }
}
