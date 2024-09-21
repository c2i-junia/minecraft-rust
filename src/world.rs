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
    let dirt_material = materials.add(Color::srgb(0.5, 0.25, 0.0));

    let approx_total_blocks = 5000;
    let bound = ((approx_total_blocks as f32).sqrt().floor() / 2.0).round() as i32;

    println!("Bound: {}", bound);

    // Génération d'un seed aléatoire
    let mut rng = rand::thread_rng();
    let seed: u32 = rng.gen();

    let perlin = Perlin::new(seed);
    println!("Seed utilisée: {}", seed);

    let scale = 0.1;

    let max_perlin_height = 10.0;

    // Boucle pour générer les blocs avec variation de hauteur
    for i in -bound..bound {
        for j in -bound..bound {
            // Générer une hauteur en utilisant le bruit de Perlin
            let perlin_height =
                perlin.get([i as f64 * scale, j as f64 * scale]) * max_perlin_height;
            let perlin_height = perlin_height.round() as i32; // Arrondir à des hauteurs entières

            // Générer les couches de blocs jusqu'à la couche y = -10
            for y in -10..=perlin_height {
                let material = if y == perlin_height {
                    grass_material.clone() // Le bloc du dessus est de l'herbe
                } else {
                    dirt_material.clone() // Les couches inférieures sont de la terre
                };

                // Placer chaque bloc à la bonne hauteur
                commands.spawn(PbrBundle {
                    mesh: cube_mesh.clone(),
                    material,
                    transform: Transform::from_translation(Vec3::new(i as f32, y as f32, j as f32)),
                    ..Default::default()
                });
            }
        }
    }
}
