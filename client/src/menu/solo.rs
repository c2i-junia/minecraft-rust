use bevy::{color::palettes::css::CRIMSON, prelude::{BuildChildren, ButtonBundle, Commands, Component, NodeBundle, TextBundle}, text::TextStyle, ui::{AlignItems, FlexDirection, JustifyContent, Style, UiRect, Val}};

use super::{MenuButtonAction, NORMAL_BUTTON};

// Tag component used to tag entities added on the play menu screen
#[derive(Component)]
pub struct OnSoloMenuScreen;

pub fn play_menu_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };

    let button_text_style = TextStyle {
        font_size: 33.0,
        color: super::TEXT_COLOR,
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
            OnSoloMenuScreen,
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
                    for (action, text) in [
                        (MenuButtonAction::NewGame, "New Game"),
                        (MenuButtonAction::LoadGame, "Load Game"),
                        (MenuButtonAction::BackToMainMenu, "Back"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..Default::default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(
                                    text,
                                    button_text_style.clone(),
                                ));
                            });
                    }
                });
        });
}
