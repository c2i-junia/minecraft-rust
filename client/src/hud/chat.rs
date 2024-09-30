use crate::network::{send_chat_message, CachedChatConversation};
use crate::{
    keyboard::{is_action_just_pressed, keyboard_clear_input},
    UiDialog,
};
use bevy::prelude::*;
use bevy_renet::renet::RenetClient;
use bevy_simple_text_input::*;

#[derive(Component)]
pub struct ChatRoot;

#[derive(Component)]
pub struct ChatDisplay;

#[derive(Component)]
pub struct ChatInput;

const CHAT_COLOR: Color = Color::srgba(0., 0., 0., 0.6);
const CHAT_SIZE: f32 = 17.;
const CHAT_MAX_MESSAGES: usize = 2;

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
                    max_height: Val::Px((CHAT_MAX_MESSAGES as f32 + 20.) * CHAT_SIZE),
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
    ),
    mut last_render_ts: Local<u64>,
    mut event: EventReader<TextInputSubmitEvent>,
    mut commands: Commands,
) {
    let (cached_conv, asset_server, mut client, mut keyboard_input) = resources;
    let (mut text_query, mut visibility_query, parent_query) = queries;

    let (entity_check, mut inactive, mut value) = text_query.single_mut();
    let mut vis = visibility_query.single_mut();
    let (parent, children) = parent_query.single();

    if is_action_just_pressed(crate::keyboard::GameAction::OpenChat, &keyboard_input) {
        inactive.0 = false;
        *vis = Visibility::Visible;
    }

    if *vis == Visibility::Visible {
        if is_action_just_pressed(crate::keyboard::GameAction::Escape, &keyboard_input) {
            *vis = Visibility::Hidden;
            *value = TextInputValue("".to_string());
            *inactive = TextInputInactive(true);
        }
        keyboard_clear_input(&mut keyboard_input);
    }

    if let Some(conv) = &cached_conv.data {
        for message in &conv.messages {

            // If message too old, don't render
            if message.date <= *last_render_ts {
                continue;
            }

            *last_render_ts = message.date;

            let msg = commands
                .spawn(TextBundle {
                    text: Text::from_section(
                        format!("<{}> : {}", message.author_name, message.content),
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

    *vis = Visibility::Hidden;
    *inactive = TextInputInactive(true);

    for message in event.read() {
        if entity_check == message.entity {
            send_chat_message(&mut client, &message.value);
        }
    }
}
