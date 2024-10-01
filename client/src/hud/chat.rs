use crate::input::keyboard::is_action_just_released;
use crate::network::{send_chat_message, CachedChatConversation};
use crate::{keyboard::is_action_just_pressed, UiDialog};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use bevy_simple_text_input::*;

#[derive(Component)]
pub struct ChatRoot;

#[derive(Component)]
pub struct ChatDisplay;

#[derive(Component)]
pub struct ChatInput;

#[derive(Component)]
pub struct MessageAnimator {
    created_ts: u64,
}

const CHAT_COLOR: Color = Color::srgba(0., 0., 0., 0.6);
const CHAT_SIZE: f32 = 17.;
const CHAT_MAX_MESSAGES: usize = 2;

// Time in ms
const ANIMATION_BEGIN_FADE: u64 = 5_000;
const ANIMATION_HIDE: u64 = 2_000;

pub fn setup_chat(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            Name::new("ChatRoot"),
            StateScoped(crate::GameState::Game),
            ChatRoot,
            UiDialog,
            NodeBundle {
                background_color: BackgroundColor(CHAT_COLOR),
                visibility: Visibility::Hidden,
                style: Style {
                    display: Display::Flex,
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(0.),
                    max_height: Val::Px((CHAT_MAX_MESSAGES as f32 + 20.) * CHAT_SIZE),
                    width: Val::Vw(20.),
                    left: Val::Percent(0.),
                    column_gap: Val::Px(0.),
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

pub fn render_chat(
    resources: (
        Res<CachedChatConversation>,
        Res<AssetServer>,
        ResMut<RenetClient>,
        ResMut<ButtonInput<KeyCode>>,
    ),
    queries: (
        Query<(Entity, &mut TextInputInactive, &mut TextInputValue), With<ChatInput>>,
        Query<&mut Visibility, With<ChatRoot>>,
        Query<(Entity, &Children), With<ChatDisplay>>,
        Query<
            (
                Entity,
                &mut BackgroundColor,
                &mut Text,
                &mut Visibility,
                &MessageAnimator,
            ),
            Without<ChatRoot>,
        >,
    ),
    mut last_render_ts: Local<u64>,
    mut event: EventReader<TextInputSubmitEvent>,
    mut commands: Commands,
) {
    let (cached_conv, asset_server, mut client, mut keyboard_input) = resources;
    let (mut text_query, mut visibility_query, parent_query, mut animation_query) = queries;

    let (entity_check, mut inactive, mut value) = text_query.single_mut();
    let mut visibility = visibility_query.single_mut();
    let (parent, children) = parent_query.single();

    if is_action_just_released(crate::keyboard::GameAction::OpenChat, &keyboard_input) {
        inactive.0 = false;
        *visibility = Visibility::Visible;
    }

    if *visibility == Visibility::Visible {
        if is_action_just_pressed(crate::keyboard::GameAction::Escape, &keyboard_input) {
            *visibility = Visibility::Hidden;
            *value = TextInputValue("".to_string());
            *inactive = TextInputInactive(true);
        }
        keyboard_input.reset_all();
    }

    let current_ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    for (entity, mut bg, mut text, mut vis, animator) in animation_query.iter_mut() {
        let diff = current_ts - animator.created_ts;

        // Additionnally, if chat is shown, cancel animation
        if diff > ANIMATION_BEGIN_FADE + ANIMATION_HIDE || *visibility == Visibility::Visible {
            // Remove animator to reduce load, reset style, and hide element
            commands.entity(entity).remove::<MessageAnimator>();
            *vis = Visibility::Inherited;
            *bg = BackgroundColor(Color::BLACK.with_alpha(0.));
            text.sections[0].style.color = Color::WHITE;
        } else if diff > ANIMATION_BEGIN_FADE {
            // Animate linear fade
            let alpha = 1. - ((diff - ANIMATION_BEGIN_FADE) as f32 / ANIMATION_HIDE as f32);
            *bg = BackgroundColor(CHAT_COLOR.with_alpha(0.6 * alpha));
            text.sections[0].style.color = Color::WHITE.with_alpha(alpha);
        }
    }

    if let Some(conv) = &cached_conv.data {
        for message in &conv.messages {
            // If message too old, don't render
            if message.date <= *last_render_ts {
                continue;
            }

            *last_render_ts = message.date;

            let msg = commands
                .spawn((
                    MessageAnimator {
                        created_ts: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_millis() as u64,
                    },
                    TextBundle {
                        text: Text::from_section(
                            format!("<{}> : {}", message.author_name, message.content),
                            TextStyle {
                                font: asset_server.load("fonts/gohu.ttf"),
                                font_size: 17.,
                                color: Color::WHITE,
                            }
                            .clone(),
                        ),
                        visibility: Visibility::Visible,
                        background_color: BackgroundColor(CHAT_COLOR),
                        ..Default::default()
                    },
                ))
                .id();

            commands.entity(parent).push_children(&[msg]);
        }

        // Prevents too much messages from building up on screen
        if children.len() > CHAT_MAX_MESSAGES {
            for i in children.len()..CHAT_MAX_MESSAGES {
                commands.entity(parent).remove_children(&[children[i]]);
                commands.entity(children[i]).despawn();
            }
        }
    }

    if event.is_empty() {
        return;
    }

    *visibility = Visibility::Hidden;
    *inactive = TextInputInactive(true);

    for message in event.read() {
        if entity_check == message.entity {
            send_chat_message(&mut client, &message.value);
        }
    }
}