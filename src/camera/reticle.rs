use bevy::prelude::*;

pub fn spawn_reticle(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(5.0),  // Largeur du réticule
            height: Val::Px(5.0), // Hauteur du réticule
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
            },
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        background_color: Color::WHITE.into(), // Couleur du réticule
        ..Default::default()
    });
}
