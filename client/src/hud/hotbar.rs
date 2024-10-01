use bevy::{prelude::*, ui::FocusPolicy};

use crate::{constants::{HOTBAR_BORDER, HOTBAR_CELL_SIZE, HOTBAR_PADDING, MAX_HOTBAR_SLOTS}, InventoryCell};

#[derive(Component)]
pub struct Hotbar {
    selected: u32,
}

pub fn setup_hotbar(mut commands: Commands) {
    commands
        .spawn((
            Hotbar { selected: 0 },
            NodeBundle {
                background_color: BackgroundColor(Color::srgba(0.3, 0.3, 0.3, 0.3)),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Row,
                    position_type: PositionType::Absolute,
                    bottom: Val::Px(70.),
                    width: Val::Auto,
                    padding: UiRect::ZERO,
                    border: UiRect::ZERO,
                    margin: UiRect::all(Val::Auto),
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
                        border_color: BorderColor(Color::srgb(0.3, 0.3, 0.3)),
                        focus_policy: FocusPolicy::Block,
                        style: Style {
                            width: Val::Px(HOTBAR_CELL_SIZE),
                            height: Val::Px(HOTBAR_CELL_SIZE),
                            margin: UiRect::ZERO,
                            position_type: PositionType::Relative,
                            padding: UiRect::all(Val::Px(HOTBAR_PADDING)),
                            border: UiRect::all(Val::Px(HOTBAR_BORDER)),
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
                        style: Style {
                            width: Val::Px(HOTBAR_CELL_SIZE - 2. * (HOTBAR_PADDING + HOTBAR_BORDER)),
                            position_type: PositionType::Relative,
                            ..Default::default()
                        },
                        image: UiImage::default(),
                        ..Default::default()
                    });
                });
            }
        });
}
