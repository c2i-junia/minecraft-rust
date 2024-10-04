use bevy::{
    asset::AssetServer,
    color::Color,
    prelude::{BuildChildren, Commands, NodeBundle, Res, StateScoped, TextBundle},
    text::{Text, TextStyle},
    ui::{
        AlignItems, Display, FlexDirection, JustifyContent, Style, UiRect, Val,
    },
};

use super::{MenuState, ScrollingList};

pub fn controls_menu_setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            StateScoped(MenuState::SettingsControls),
            NodeBundle {
                style: Style {
                    padding: UiRect::horizontal(Val::Vw(15.)),
                    top: Val::Px(0.),
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
            ScrollingList { offset: 0. },
        ))
        .with_children(|root| {
            root.spawn(TextBundle {
                text: Text::from_section(
                    "Keyboard Controls",
                    TextStyle {
                        font: assets.load("fonts/gohu.ttf"),
                        font_size: 24.,
                        color: Color::WHITE,
                    },
                ),
                ..Default::default()
            });
        });
}
