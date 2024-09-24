use crate::input::keyboard::{is_action_just_pressed, GameAction};
use crate::materials::MeshResource;
use crate::world::BlockMarker;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};

#[derive(Resource)]
pub struct BlockDebugWireframeSettings {
    pub is_enabled: bool,
}

pub fn toggle_wireframe_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut blocks_query: Query<&mut Handle<Mesh>, With<BlockMarker>>,
    mesh_resource: Res<MeshResource>,
    mut settings: ResMut<BlockDebugWireframeSettings>,
) {
    if is_action_just_pressed(GameAction::ToggleBlockWireframeDebugMode, &keyboard_input)
        && !settings.is_enabled
    {
        settings.is_enabled = true;
        for mut mesh_handle in blocks_query.iter_mut() {
            *mesh_handle = mesh_resource.wireframe_mesh.clone();
        }
        return;
    }

    if is_action_just_pressed(GameAction::ToggleBlockWireframeDebugMode, &keyboard_input) {
        settings.is_enabled = false;
        for mut mesh_handle in blocks_query.iter_mut() {
            *mesh_handle = mesh_resource.cube_mesh.clone();
        }
    }
}

pub fn create_wireframe_cube() -> Mesh {
    let mut mesh = Mesh::new(PrimitiveTopology::LineList, Default::default());

    // Define the 8 vertices of the cube
    let vertices = vec![
        [-0.5, -0.5, -0.5],
        [0.5, -0.5, -0.5],
        [0.5, -0.5, 0.5],
        [-0.5, -0.5, 0.5],
        [-0.5, 0.5, -0.5],
        [0.5, 0.5, -0.5],
        [0.5, 0.5, 0.5],
        [-0.5, 0.5, 0.5],
    ];

    // Define the 12 edges of the cube
    let indices: Vec<u32> = vec![
        0, 1, 1, 2, 2, 3, 3, 0, 4, 5, 5, 6, 6, 7, 7, 4, 0, 4, 1, 5, 2, 6, 3, 7,
    ];

    // Add vertices to the mesh
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);

    // Add indices to define the wireframe edges
    mesh.insert_indices(Indices::U32(indices));

    mesh
}
