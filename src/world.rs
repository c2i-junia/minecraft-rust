use bevy::prelude::*;
use bevy::prelude::{Commands, ResMut};
use noise::{NoiseFn, Perlin};
use rand::Rng;

pub fn setup_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let mut rng = rand::thread_rng();

    let cube_mesh = meshes.add(Mesh::from(Cuboid::new(1.0, 1.0, 1.0)));
    let grass_material = materials.add(Color::srgb(0.0, 0.5, 0.0));

    let approx_total_blocks = 5000;

    let bound = ((approx_total_blocks as f32).sqrt().floor() / 2.0).round() as i32;

    println!("Bound: {}", bound);

    // Génération d'un seed aléatoire
    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();

    let perlin = Perlin::new(seed);
    println!("Seed utilisée: {}", seed);

    let scale = 0.1;

    // Boucle pour générer les blocs avec variation de hauteur par bloc
    for i in -bound..bound {
        for j in -bound..bound {
            // Générer une hauteur en utilisant le bruit de Perlin
            let height = perlin.get([i as f64 * scale, j as f64 * scale]) * 5.0;

            // Arrondir la hauteur à l'entier le plus proche pour que chaque bloc soit aligné
            let height_block = height.round(); // Conversion en bloc entier

            // Placer chaque bloc à la hauteur arrondie
            commands.spawn(PbrBundle {
                mesh: cube_mesh.clone(),
                material: grass_material.clone(),
                transform: Transform::from_translation(Vec3::new(
                    i as f32,
                    height_block as f32,
                    j as f32,
                )),
                ..Default::default()
            });
        }
    }
}
