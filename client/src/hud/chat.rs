use bevy::prelude::*;
use bevy_simple_text_input::*;

use crate::{
    keyboard::{is_action_just_pressed, keyboard_clear_input},
    UiDialog,
};

#[derive(Component)]
pub struct ChatRoot;

#[derive(Component)]
pub struct ChatDisplay;

#[derive(Component)]
pub struct ChatInput;

const CHAT_COLOR: Color = Color::srgba(0., 0., 0., 0.6);
const CHAT_SIZE: f32 = 17.;
const CHAT_MAX_MESSAGES: i32 = 2;

pub fn setup_chat(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("ChatRoot"),
            StateScoped(crate::GameState::Game),
            ChatRoot,
            UiDialog,
            NodeBundle {
                background_color: BackgroundColor(CHAT_COLOR),
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.),
                    max_height: Val::Px((CHAT_MAX_MESSAGES + 20) as f32 * CHAT_SIZE),
                    width: Val::Vw(20.),
                    left: Val::Percent(0.),
                    overflow: Overflow {
                        x: OverflowAxis::Visible,
                        y: OverflowAxis::Hidden,
                    },
                    flex_direction: FlexDirection::Column,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|root| {
            root.spawn((
                ChatDisplay,
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        flex_direction: FlexDirection::Column,
                        overflow: Overflow {
                            x: OverflowAxis::Visible,
                            y: OverflowAxis::Hidden,
                        },
                        width: Val::Percent(100.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            ))
            .with_children(|d| {
                // DO NOT REMOVE !!!
                // Function send_chat has a bit of a meltdown if the ChatDisplay has no children (cuz of the Query)
                d.spawn(NodeBundle::default());
            });

            root.spawn((
                ChatInput,
                NodeBundle {
                    style: Style {
                        width: Val::Percent(100.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                bevy_simple_text_input::TextInputBundle {
                    placeholder: TextInputPlaceholder {
                        value: "Send a message...".to_string(),
                        ..Default::default()
                    },
                    text_style: TextInputTextStyle(TextStyle {
                        font: asset_server.load("fonts/gohu.ttf"),
                        font_size: 17.,
                        color: Color::WHITE,
                    }),
                    inactive: TextInputInactive(true),
                    ..Default::default()
                },
            ));
        });
}

pub fn open_chat_input(
    mut text_input: Query<&mut TextInputInactive, With<ChatInput>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut visibility: Query<&mut Visibility, With<ChatRoot>>,
) {
    if is_action_just_pressed(crate::keyboard::GameAction::OpenChat, &keyboard_input) {
        let mut input_inactive = text_input.single_mut();
        input_inactive.0 = false;
        let mut vis = visibility.single_mut();
        *vis = Visibility::Visible;
    }
}

pub fn chat_input_check(
    mut visibility: Query<&mut Visibility, With<ChatRoot>>,
    mut input: ResMut<ButtonInput<KeyCode>>,
    mut query: Query<(&mut TextInputInactive, &mut TextInputValue), With<ChatInput>>,
) {
    if visibility.single() == Visibility::Visible {
        if input.just_pressed(KeyCode::Escape) {
            let (mut inactive, mut value) = query.single_mut();
            *visibility.single_mut() = Visibility::Hidden;
            *value = TextInputValue("".to_string());
            *inactive = TextInputInactive(true);
            keyboard_clear_input(&mut input);
        }
    }
}

pub fn send_chat(
    mut event: EventReader<TextInputSubmitEvent>,
    mut input_query: Query<(Entity, &mut TextInputInactive), With<ChatInput>>,
    mut root_query: Query<&mut Visibility, With<ChatRoot>>,
    parent_query: Query<(Entity, &Children), With<ChatDisplay>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    if event.is_empty() {
        return;
    }

    let (entity_check, mut inactive) = input_query.single_mut();
    let mut vis = root_query.single_mut();
    let (parent, children) = parent_query.single();

    *vis = Visibility::Hidden;
    *inactive = TextInputInactive(true);

    for message in event.read() {
        if entity_check == message.entity {
            println!(
                "Message Sent : {:?}, total messages : {:?}",
                message.value,
                children.len()
            );
            let msg = commands
                .spawn(TextBundle {
                    text: Text::from_section(
                        message.value.clone(),
                        TextStyle {
                            font: asset_server.load("fonts/gohu.ttf"),
                            font_size: 17.,
                            color: Color::WHITE,
                        }
                        .clone(),
                    ),
                    ..Default::default()
                })
                .id();

            commands.entity(parent).push_children(&[msg]);

            // Prevents history from containing more than 20 messages
            if children.len() as i32 > CHAT_MAX_MESSAGES {
                commands.entity(children[0]).despawn();
            }
        }
    }
}
