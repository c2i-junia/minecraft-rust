use crate::block_debug_wireframe::create_wireframe_cube;
use crate::constants::{BASE_ROUGHNESS, CUBE_SIZE};
use crate::world::Block;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct MaterialResource {
    pub materials: HashMap<Block, Handle<StandardMaterial>>,
}

pub fn setup_materials(mut commands: Commands, asset_server: Res<AssetServer>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut material_resource = MaterialResource { ..default() };

    // Root directory for asset server : /assets/
    // TODO : atlas textures (currently only supports 1 texture per cube, for all 6 faces)
    let grass_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/grass.png")),
        base_color: Color::srgb(0.14, 0.7, 0.2),
        perceptual_roughness: BASE_ROUGHNESS,
        ..default()
    });     // MC's grass texture is grey and tinted via a colormap according to biome
            // Don't have the knowledge to do that atm so used constant "grass green" color instead
            // Modifying color based on noise generation values could be interesting tho

    let dirt_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/dirt.png")),
        perceptual_roughness: BASE_ROUGHNESS,
        ..default()
    });
    let stone_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/stone.png")),
        perceptual_roughness: BASE_ROUGHNESS,
        ..default()
    });
    let bedrock_material = materials.add(StandardMaterial {
        base_color_texture: Some(asset_server.load("textures/bedrock.png")),
        perceptual_roughness: BASE_ROUGHNESS,
        ..default()
    });
    

    material_resource
        .materials
        .insert(Block::Dirt, dirt_material);
    material_resource
        .materials
        .insert(Block::Grass, grass_material);
    material_resource
        .materials
        .insert(Block::Stone, stone_material);
    material_resource
        .materials
        .insert(Block::Bedrock, bedrock_material);

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
