use crate::{
    constants::{CELESTIAL_DISTANCE, CELESTIAL_SIZE, DAY_DURATION},
    materials::MaterialResource,
    world::GlobalMaterial,
    Player,
};
use bevy::{
    log::tracing_subscriber::fmt::time, pbr::{NotShadowCaster, NotShadowReceiver}, prelude::*
};
use std::f32::consts::PI;

//
#[derive(Component)]
pub struct CelestialRoot;

// Main light source : the sun
#[derive(Component)]
pub struct SunLight;

// Secondary main light source : the moon
#[derive(Component)]
pub struct MoonLight;

pub fn setup_main_lighting(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    material_resource: Res<MaterialResource>,
    player: Query<Entity, With<Player>>,
) {
    // No fancy stuff ; Only acts as an anchor to move celestial bodies easily
    let celestial_root = commands
        .spawn((CelestialRoot, SpatialBundle::default()))
        .id();

    let mut light_transform = Transform::from_translation(Vec3::new(0., 0., 0.));

    let sun_light = commands
        .spawn((
            SunLight,
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 5000.,
                    shadows_enabled: true,
                    ..Default::default()
                },
                transform: light_transform.clone(),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Rectangle::new(CELESTIAL_SIZE, CELESTIAL_SIZE)),
                    material: material_resource
                        .global_materials
                        .get(&GlobalMaterial::Sun)
                        .expect("Sun material not found !")
                        .clone(),
                    transform: Transform {
                        translation: Vec3::new(0., 0., CELESTIAL_DISTANCE),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                NotShadowCaster,
                NotShadowReceiver,
            ));
        })
        .id();
    light_transform.rotate_y(PI);

    let moon_light = commands
        .spawn((
            MoonLight,
            DirectionalLightBundle {
                directional_light: DirectionalLight {
                    illuminance: 500.,
                    color: Color::Srgba(Srgba::hex("c9d2de").unwrap()),
                    shadows_enabled: true,

                    ..Default::default()
                },
                transform: light_transform.clone(),
                ..Default::default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                PbrBundle {
                    mesh: meshes.add(Rectangle::new(CELESTIAL_SIZE, CELESTIAL_SIZE)),
                    material: material_resource
                        .global_materials
                        .get(&GlobalMaterial::Moon)
                        .expect("Moon material not found !")
                        .clone(),
                    transform: Transform {
                        translation: Vec3::new(0., 0., CELESTIAL_DISTANCE),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                NotShadowCaster,
                NotShadowReceiver,
            ));
        })
        .id();

    commands
        .entity(celestial_root)
        .push_children(&[sun_light, moon_light]);

    commands.entity(player.single()).add_child(celestial_root);
}

pub fn update_celestial_bodies(
    mut query: Query<&mut Transform, With<CelestialRoot>>,
    time: Res<Time>,
) {
    for mut tr in &mut query {
        tr.rotate(Quat::from_rotation_x((2. * PI * time.delta_seconds()) / DAY_DURATION));
    }
}
