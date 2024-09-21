use bevy::prelude::*;
use bevy::prelude::{Commands, ResMut};
use rand::Rng;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    /*
    // Spawn the platform
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(20.0, 1.0, 20.0))),
        material: materials.add(Color::srgb(0.75, 0.75, 0.75)),
        transform: Transform::from_translation(Vec3::new(0.0, -0.5, 0.0)),
        ..Default::default()
    });
     */

    let mut rng = rand::thread_rng();

    for i in -10..11 {
        for j in -10..11 {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0))),
                material: materials.add(Color::srgb(rng.gen(), rng.gen(), rng.gen())),
                transform: Transform::from_translation(Vec3::new(i as f32, -0.5, j as f32)),
                ..Default::default()
            });
        }
    }

}
