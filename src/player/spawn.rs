use crate::camera::CameraController;
use crate::input::keyboard::*;
use crate::player::{Player, ViewMode};
use bevy::prelude::*;

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 2.0, 1.0))),
            material: materials.add(Color::srgba(1.0, 0.0, 0.0, 0.0)),
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
            ..Default::default()
        })
        .insert(Player::new());
}
