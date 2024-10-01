use bevy::{
    app::AppExit,
    asset::AssetServer,
    color::{Alpha, Color},
    core::Name,
    input::ButtonInput,
    prelude::{
        BuildChildren, ButtonBundle, Commands, Component, EventWriter, KeyCode, NodeBundle, Query,
        Res, ResMut, StateScoped, TextBundle, Visibility, With,
    },
    text::{Text, TextStyle},
    ui::{
        AlignItems, BackgroundColor, BorderColor, Display, FlexDirection, FocusPolicy, Interaction,
        JustifyContent, Style, UiRect, Val, ZIndex,
    },
};

use crate::{keyboard::is_action_just_pressed, GameState};

use super::UiDialog;

#[derive(Component)]
pub struct PauseMenu;

#[derive(Component)]
pub enum PauseButtonAction {
    Resume,
    Quit,
}

pub fn setup_pause_menu(mut commands: Commands, assets: Res<AssetServer>) {
    commands
        .spawn((
            PauseMenu,
            UiDialog,
            Name::new("PauseMenu"),
            StateScoped(GameState::Game),
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.6)),
                style: Style {
                    width: Val::Vw(100.),
                    height: Val::Vh(100.),
                    display: Display::Flex,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                focus_policy: FocusPolicy::Block,
                visibility: Visibility::Hidden,
                z_index: ZIndex::Global(5),
                ..Default::default()
            },
        ))
        .with_children(|root| {
            root.spawn(NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::SpaceAround,
                    height: Val::Vh(40.),
                    min_width: Val::Vw(40.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .with_children(|wrapper| {
                for (msg, action) in [
                    ("Resume", PauseButtonAction::Resume),
                    ("Quit", PauseButtonAction::Quit),
                ] {
                    wrapper
                        .spawn((
                            action,
                            ButtonBundle {
                                background_color: BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                                border_color: BorderColor(Color::BLACK),
                                style: Style {
                                    width: Val::Percent(100.),
                                    border: UiRect::all(Val::Px(3.)),
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        ))
                        .with_children(|btn| {
                            btn.spawn(TextBundle {
                                text: Text::from_section(
                                    msg,
                                    TextStyle {
                                        font: assets.load("fonts/gohu.ttf"),
                                        font_size: 20.,
                                        color: Color::WHITE,
                                    },
                                ),
                                ..Default::default()
                            });
                        });
                }
            });
        });
}

pub fn render_pause_menu(
    queries: (
        Query<(&PauseButtonAction, &mut BorderColor, &Interaction)>,
        Query<&mut Visibility, With<PauseMenu>>,
    ),
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut exit: EventWriter<AppExit>,
) {
    let (mut button, mut visibility) = queries;
    let mut vis = visibility.single_mut();

    if is_action_just_pressed(crate::keyboard::GameAction::Escape, &input) {
        *vis = match *vis {
            Visibility::Visible | Visibility::Inherited => Visibility::Hidden,
            Visibility::Hidden => Visibility::Visible,
        }
    }

    if *vis != Visibility::Visible {
        return;
    }

    for (action, mut bcolor, interaction) in button.iter_mut() {
        match *interaction {
            Interaction::Pressed => match *action {
                PauseButtonAction::Quit => {
                    exit.send(AppExit::Success);
                }
                PauseButtonAction::Resume => {
                    *vis = Visibility::Hidden;
                }
            },
            Interaction::Hovered => {
                bcolor.0 = Color::WHITE;
            }
            Interaction::None => {
                bcolor.0 = Color::BLACK;
            }
        }
    }

    input.reset_all();
}
