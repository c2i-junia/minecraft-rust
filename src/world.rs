use bevy::prelude::*;
use bevy::prelude::{Commands, ResMut};
// use rand::Rng;

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

    // let mut rng = rand::thread_rng();

    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let grass_material = materials.add(Color::srgb(0.0, 0.5, 0.0));

    let approx_total_blocks = 5000;

    let bound = ((approx_total_blocks as f32).sqrt().floor() / 2.0).round() as i32;

    println!("Bound: {}", bound);

    for i in -bound..bound {
        for j in -bound..bound {
            commands.spawn(PbrBundle {
                mesh: cube_mesh.clone(),
                material: grass_material.clone(),
                transform: Transform::from_translation(Vec3::new(i as f32, -0.5, j as f32)),
                ..Default::default()
            });
        }
    }
}
