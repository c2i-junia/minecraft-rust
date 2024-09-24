use crate::player::Player;
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(0.8, 1.8, 0.8))),
            material: materials.add(Color::srgba(1.0, 0.0, 0.0, 0.0)),
            transform: Transform::from_translation(Vec3::new(7.5, 100.0, 7.5)),
            ..Default::default()
        })
        .insert(Player::new());
}
