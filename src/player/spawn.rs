use crate::player::Player;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = Player::new();

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                player.width,
                player.height,
                player.width,
            ))),
            material: materials.add(Color::srgba(1.0, 0.0, 0.0, 0.0)),
            transform: Transform::from_translation(Vec3::new(7.5, 100.0, 7.5)),
            ..Default::default()
        })
        .insert(player);
}
