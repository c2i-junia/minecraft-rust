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
    cube_mesh: Handle<Mesh>,
}

pub fn setup_cube_mesh(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let cube_mesh = Mesh::from(Cuboid::new(1.0, 1.0, 1.0));
    let cube_handle = meshes.add(cube_mesh);

    commands.insert_resource(MeshResource {
        cube_mesh: cube_handle,
    });
}
