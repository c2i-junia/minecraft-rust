use crate::{
    keyboard::{get_action_keys, GameAction},
    player::Player,
    ItemsType,
};
use bevy::prelude::*;

// Marker for Inventory root
#[derive(Component)]
pub struct InventoryRoot;

// Main inventory dialog
#[derive(Component)]
pub struct InventoryDialog;

#[derive(Component)]
pub struct InventoryGrid;

pub fn setup_inventory(mut commands: Commands) {
    // Inventory root : root container for the inventory
    let root = commands
        .spawn((
            InventoryRoot,
            NodeBundle {
                background_color: BackgroundColor(Color::BLACK.with_alpha(0.4)),
                // Z-index of 1 : displayed above game, but under everything else
                z_index: ZIndex::Global(1),
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
            ..Default::default()
        })
        .id();

    let inventory_grid = commands
        .spawn((
            InventoryGrid,
            NodeBundle {
                style: Style {
                    display: Display::Grid,
                    grid_template_columns: RepeatedGridTrack::auto(9),
                    margin: UiRect::all(Val::Px(20.)),
                    padding: UiRect::ZERO,
                    border: UiRect::all(Val::Px(1.)),
                    ..Default::default()
                },
                border_color: BorderColor(Color::BLACK),
                ..Default::default()
            },
        ))
        .with_children(|builder| {
            for i in 0..27 {
                builder
                    .spawn(ButtonBundle {
                        border_color: BorderColor(Color::BLACK),
                        style: Style {
                            width: Val::Px(50.),
                            height: Val::Px(50.),
                            margin: UiRect::ZERO,
                            border: UiRect::all(Val::Px(1.)),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .with_children(|btn| {
                        btn.spawn(TextBundle::from_section(
                            format!("{:?}", i),
                            TextStyle {
                                font_size: 15.,
                                ..Default::default()
                            },
                        ));
                    });
            }
        })
        .id();

    commands
        .entity(dialog)
        .push_children(&[inventory_title, inventory_grid]);

    commands.entity(root).push_children(&[dialog]);

    let t = ItemsType::Bedrock as i32;
    let u = ItemsType::try_from(1).unwrap();
    println!("t : {:?}, u : {:?}", t, u);
}

// Open inventory when E key is pressed
pub fn toggle_inventory(
    mut q: Query<&mut Visibility, With<InventoryRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    let keys = get_action_keys(GameAction::OpenInventory);
    for key in keys {
        if kbd.just_pressed(key) {
            let mut vis = q.single_mut();
            *vis = match *vis {
                Visibility::Hidden => Visibility::Visible,
                _ => Visibility::Hidden,
            };
        }
    }
}

pub fn inventory_text_update_system(
    player: Query<&Player>,
    mut query: Query<&mut Text, With<InventoryGrid>>,
) {
    for mut text in query.iter_mut() {
        let player = player.single();
        // Check if inventory is empty
        if player.inventory.is_empty() {
            text.sections[0].value = "Inventory: Empty".to_string();
            return;
        }
        // Update inventory text
        text.sections[0].value = format!("Inventory: {:?}", player.inventory);
    }
}
