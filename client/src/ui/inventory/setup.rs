use super::UiDialog;
use crate::constants::{
    HOTBAR_BORDER, HOTBAR_CELL_SIZE, HOTBAR_PADDING, MAX_HOTBAR_SLOTS, MAX_INVENTORY_SLOTS,
};
use crate::ui::{FloatingStack, InventoryCell, InventoryDialog, InventoryRoot};
use bevy::{prelude::*, ui::FocusPolicy};

pub fn setup_inventory(mut commands: Commands) {
    // Inventory root : root container for the inventory
    let root = commands
        .spawn((
            UiDialog,
            InventoryRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.4)),
                // Z-index of 2 : displayed above game & HUD, but under everything else
                z_index: ZIndex::Global(2),
                visibility: Visibility::Hidden,
                style: Style {
                    position_type: PositionType::Absolute,
                    // Cover whole screen as a dark backdrop
                    left: Val::Percent(0.),
                    right: Val::Percent(0.),
                    bottom: Val::Percent(0.),
                    top: Val::Percent(0.),
                    // Align children at its center
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .id();

    let dialog = commands
        .spawn((
            InventoryDialog,
            NodeBundle {
                background_color: BackgroundColor(Color::srgb(0.4, 0.4, 0.4)),
                border_radius: BorderRadius::all(Val::Percent(10.)),
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    padding: UiRect::all(Val::Percent(7.)),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    let inventory_title = commands
        .spawn(TextBundle {
            text: Text::from_section(
                "Inventory",
                TextStyle {
                    font_size: 24.,
                    ..Default::default()
                },
            ),
            style: Style {
                align_content: AlignContent::Center,
                ..Default::default()
            },
            ..Default::default()
        })
        .id();

    let inventory_grid = commands
        .spawn(NodeBundle {
            style: Style {
                display: Display::Grid,
                grid_template_columns: RepeatedGridTrack::auto(9),
                margin: UiRect::all(Val::Px(10.)),
                position_type: PositionType::Relative,
                ..Default::default()
            },
            border_color: BorderColor(Color::BLACK),
            ..Default::default()
        })
        .with_children(|builder| {
            for i in MAX_HOTBAR_SLOTS..MAX_INVENTORY_SLOTS {
                builder
                    .spawn((
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
                                width: Val::Px(
                                    HOTBAR_CELL_SIZE - 2. * (HOTBAR_PADDING + HOTBAR_BORDER),
                                ),
                                position_type: PositionType::Relative,
                                ..Default::default()
                            },
                            image: UiImage::default(),
                            ..Default::default()
                        });
                    });
            }
        })
        .id();

    let floating_stack = commands
        .spawn((
            FloatingStack { items: None },
            NodeBundle {
                focus_policy: FocusPolicy::Pass,
                style: Style {
                    width: Val::Px(20.),
                    height: Val::Px(20.),
                    position_type: PositionType::Absolute,
                    ..Default::default()
                },
                ..Default::default()
            },
        ))
        .with_children(|btn| {
            btn.spawn(TextBundle::from_section(
                "",
                TextStyle {
                    font_size: 15.,
                    ..Default::default()
                },
            ));
            btn.spawn(ImageBundle {
                z_index: ZIndex::Local(-1),
                style: Style {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(0.),
                    right: Val::Percent(0.),
                    bottom: Val::Percent(0.),
                    top: Val::Percent(0.),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .id();

    commands
        .entity(dialog)
        .push_children(&[inventory_title, inventory_grid]);

    commands
        .entity(root)
        .push_children(&[dialog, floating_stack]);
}
