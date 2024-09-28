use bevy::{prelude::*, ui::FocusPolicy};

use crate::{constants::MAX_HOTBAR_SLOTS, InventoryCell};

#[derive(Component)]
pub struct Hotbar {
    selected: u32,
}

pub fn setup_hotbar(mut commands: Commands) {
    commands
        .spawn((
            Hotbar { selected: 0 },
            NodeBundle {
                // background_color: BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
                border_color: BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                border_radius: BorderRadius::all(Val::Px(20.)),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    top: Val::Px(70.),
                    width: Val::Auto,
                    padding: UiRect::all(Val::Px(5.)),
                    border: UiRect::all(Val::Px(5.)),
                    ..Default::default()
                },
                z_index: ZIndex::Global(1),
                ..Default::default()
            },
        ))
        .with_children(|bar| {
            for i in 0..MAX_HOTBAR_SLOTS {
                bar.spawn((
                    InventoryCell { id: i },
                    ButtonBundle {
                        border_color: BorderColor(Color::BLACK),
                        focus_policy: FocusPolicy::Block,
                        style: Style {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            margin: UiRect::ZERO,
                            padding: UiRect::all(Val::Percent(10.)),
                            border: UiRect::all(Val::Px(1.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                ))
                .with_children(|btn| {
                    btn.spawn(TextBundle {
                        text: Text::from_section(
                            "Test",
                            TextStyle {
                                font_size: 15.,
                                ..Default::default()
                            },
                        ),
                        style: Style {
                            position_type: PositionType::Absolute,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                    btn.spawn(ImageBundle {
                        z_index: ZIndex::Local(-1),
                        style: Style::default(),
                        ..Default::default()
                    });
                });
            }
        });
}
