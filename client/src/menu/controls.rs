use bevy::{
    asset::{AssetServer, Handle},
    color::{palettes::css, Color},
    prelude::{BuildChildren, ButtonBundle, Commands, NodeBundle, Res, StateScoped, TextBundle},
    text::{Font, Text, TextStyle},
    ui::{
        AlignItems, BackgroundColor, BorderRadius, Display, FlexDirection, JustifyContent, Overflow, PositionType, RepeatedGridTrack, Style, UiRect, Val
    },
};

use crate::{GameState, KeyMap};

use super::{MenuButtonAction, MenuState, ScrollingList};

pub fn controls_menu_setup(mut commands: Commands, assets: Res<AssetServer>, key_map: Res<KeyMap>) {
    let font: Handle<Font> = assets.load("fonts/gohu.ttf");

    commands
        .spawn((
            StateScoped(MenuState::SettingsControls),
            NodeBundle {
                style: Style {
                    padding: UiRect::horizontal(Val::Vw(15.)),
                    top: Val::Px(0.),
                    display: Display::Flex,
                    width: Val::Vw(100.),
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(10.),
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                ButtonBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        top: Val::Px(10.),
                        left: Val::Px(10.),
                        padding: UiRect::all(Val::Px(5.)),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                MenuButtonAction::BackToSettings,
            ))
            .with_children(|btn| {
                btn.spawn(TextBundle {
                    text: Text::from_section(
                        "Back",
                        TextStyle {
                            font: font.clone(),
                            font_size: 21.,
                            color: Color::WHITE,
                        },
                    ),
                    ..Default::default()
                });
            });

            root.spawn(NodeBundle {
                style: Style {
                    overflow: Overflow::clip_y(),
                    height: Val::Vh(100.),
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|wrapper| {
                wrapper
                    .spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: FlexDirection::Column,
                                align_items: AlignItems::Center,
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        ScrollingList { position: 0. },
                    ))
                    .with_children(|list| {
                        list.spawn(TextBundle {
                            text: Text::from_section(
                                "Keyboard Controls",
                                TextStyle {
                                    font: font.clone(),
                                    font_size: 36.,
                                    color: Color::WHITE,
                                },
                            ),
                            style: Style {
                                margin: UiRect::vertical(Val::Px(20.)),
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                        for (action, keys) in &key_map.map {
                            list.spawn((
                                ButtonBundle {
                                    style: Style {
                                        display: Display::Grid,
                                        width: Val::Percent(100.),
                                        padding: UiRect::all(Val::Px(10.)),
                                        grid_template_columns: RepeatedGridTrack::flex(2, 1.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                                StateScoped(GameState::Menu),
                            ))
                            .with_children(|line| {
                                line.spawn(TextBundle {
                                    text: Text::from_section(
                                        format!("{:?}", action),
                                        TextStyle {
                                            font: font.clone(),
                                            font_size: 24.,
                                            color: Color::WHITE,
                                        },
                                    ),
                                    ..Default::default()
                                });

                                line.spawn(NodeBundle {
                                    style: Style {
                                        flex_direction: FlexDirection::RowReverse,
                                        column_gap: Val::Px(15.),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                }).with_children(|keys_display| {
                                    for key in keys {
                                        keys_display.spawn(NodeBundle {
                                            background_color: BackgroundColor(Color::Srgba(css::BLUE_VIOLET)),
                                            border_radius: BorderRadius::all(Val::Px(10.)),
                                            style: Style {
                                                padding: UiRect::horizontal(Val::Px(10.)),
                                                ..Default::default()
                                            },
                                            ..Default::default()
                                        }).with_children(|k| {
                                            k.spawn(TextBundle {
                                                text: Text::from_section(format!("{:?}", key), TextStyle {
                                                    font: font.clone(),
                                                    font_size: 21.,
                                                    color: Color::WHITE,
                                                }),
                                                ..Default::default()
                                            });
                                        });
                                    }
                                });
                            });
                        }
                    });
            });
        });
}
