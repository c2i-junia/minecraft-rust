use crate::block_debug_wireframe::create_wireframe_cube;
use crate::constants::{BASE_ROUGHNESS, BASE_SPECULAR_HIGHLIGHT, CUBE_SIZE};
use crate::world::{Block, GlobalMaterial};
use bevy::prelude::*;
use bevy::render::render_resource::Face;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct MaterialResource {
    pub block_materials: HashMap<Block, Handle<StandardMaterial>>,
    pub global_materials : HashMap<GlobalMaterial, Handle<StandardMaterial>>,
}

pub fn setup_materials(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut material_resource = MaterialResource { ..default() };

    let sun_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1., 0.95, 0.1),
        emissive: LinearRgba::new(1., 0.95, 0.1, 0.5),
        emissive_exposure_weight: 0.5,
        cull_mode: Some(Face::Front),
        ..Default::default()
    });

    let moon_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        emissive: LinearRgba::WHITE,
        emissive_exposure_weight: 0.5,
        cull_mode: Some(Face::Front),
        ..Default::default()
    });

    // Root directory for asset server : /assets/
    // TODO : atlas textures (currently only supports 1 texture per cube, for all 6 faces)
    let grass_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/grass.png")),
        base_color: Color::srgb(0.2, 0.85, 0.3),
        perceptual_roughness: BASE_ROUGHNESS,
        reflectance: BASE_SPECULAR_HIGHLIGHT,
        ..default()
    });     // MC's grass texture is grey and tinted via a colormap according to biome
            // Don't have the knowledge to do that atm so used constant "grass green" color instead
            // Modifying color based on noise generation values could be interesting tho

    let dirt_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/dirt.png")),
        perceptual_roughness: BASE_ROUGHNESS,
        reflectance: BASE_SPECULAR_HIGHLIGHT,
        ..default()
    });
    let stone_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/stone.png")),
        perceptual_roughness: BASE_ROUGHNESS,
        reflectance: BASE_SPECULAR_HIGHLIGHT,
        ..default()
    });
    let bedrock_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/bedrock.png")),
        perceptual_roughness: BASE_ROUGHNESS,
        reflectance: BASE_SPECULAR_HIGHLIGHT,
        ..default()
    });
    

    material_resource
        .block_materials
        .insert(Block::Dirt, dirt_material);
    material_resource
        .block_materials
        .insert(Block::Grass, grass_material);
    material_resource
        .block_materials
        .insert(Block::Stone, stone_material);
    material_resource
        .block_materials
        .insert(Block::Bedrock, bedrock_material);

    material_resource
        .global_materials
        .insert(GlobalMaterial::Sun, sun_material);
    material_resource
        .global_materials
        .insert(GlobalMaterial::Moon, moon_material);

    commands.insert_resource(material_resource);
}

#[derive(Resource)]
pub struct MeshResource {
    pub cube_mesh: Handle<Mesh>,
    pub wireframe_mesh: Handle<Mesh>,
}

pub fn setup_cube_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let cube_mesh = Mesh::from(Cuboid::new(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE));
    let cube_handle = meshes.add(cube_mesh);

    let cube_wireframe_mesh = create_wireframe_cube();
    let cube_wireframe_handle = meshes.add(cube_wireframe_mesh);

    commands.insert_resource(MeshResource {
        cube_mesh: cube_handle,
        wireframe_mesh: cube_wireframe_handle,
    });
}
