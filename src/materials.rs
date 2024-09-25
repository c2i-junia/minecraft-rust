use crate::block_debug_wireframe::create_wireframe_cube;
use crate::constants::CUBE_SIZE;
use crate::world::Block;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource, Default)]
pub struct MaterialResource {
    pub materials: HashMap<Block, Handle<StandardMaterial>>,
}

pub fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let mut material_resource = MaterialResource { ..default() };

    let grass_material = materials.add(Color::srgb(0.0, 0.5, 0.0));
    let dirt_material = materials.add(Color::srgb(0.5, 0.25, 0.0));

    material_resource
        .materials
        .insert(Block::Dirt, dirt_material);

    material_resource
        .materials
        .insert(Block::Grass, grass_material);

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
