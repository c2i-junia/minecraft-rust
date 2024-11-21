use crate::constants::GRASS_COLOR;
use shared::world::{BlockData, BlockId};

/// Specifies which position in the voxel this face occupies
///
/// These faces do not render under certain circumstances to preserve resources
pub enum FaceDirection {
    /// The face is at the top of the voxel. Won't render if the block above this one is full
    Top,
    /// The face is at the bottom of the voxel. Won't render if the block under this one is full
    Bottom,
    /// The face is at the front of the voxel. Won't render if the block in front of this one is full
    Front,
    /// The face is at the back of the voxel. Won't render if the block behind this one is full
    Back,
    /// The face is at the right of the voxel. Won't render if the block at the right of this one is full
    Right,
    /// The face is at the left of the voxel. Won't render if the block at the left of this one is full
    Left,
    // /// The face is inside of the voxel. Will always render, except if the block is hidden on _all_ 6 sides
    // Inset,
}

/// Structure for cube voxel rendering
pub struct Face {
    pub direction: FaceDirection,
    pub vertices: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
    pub normals: Vec<[f32; 3]>,
    pub colors: Vec<[f32; 4]>,
    pub uvs: Vec<[f32; 2]>,
}

/// Structure for voxel rendering
pub struct VoxelShape {
    pub faces: Vec<Face>,
}

impl VoxelShape {
    /// Creates a VoxelShape based on the given BlockData
    pub fn create_from_block(block: &BlockData) -> VoxelShape {
        match block.id {
            BlockId::Grass => {
                let mut shape = Self::full_cube();
                for face in shape.faces.iter_mut() {
                    for col in face.colors.iter_mut() {
                        *col = GRASS_COLOR;
                    }
                }
                shape
            }
            _ => Self::full_cube(),
        }
    }

    pub fn full_cube() -> Self {
        VoxelShape {
            faces: vec![
                Face {
                    direction: FaceDirection::Top,
                    vertices: vec![[1., 1., 0.], [0., 1., 0.], [0., 1., 1.], [1., 1., 1.]],
                    indices: vec![0, 1, 2, 2, 3, 0],
                    normals: vec![
                        [0.0, 1.0, 0.0],
                        [0.0, 1.0, 0.0],
                        [0.0, 1.0, 0.0],
                        [0.0, 1.0, 0.0],
                    ],
                    colors: vec![
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    ],
                    uvs: vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
                },
                Face {
                    direction: FaceDirection::Bottom,
                    vertices: vec![[0., 0., 0.], [1., 0., 0.], [1., 0., 1.], [0., 0., 1.]],
                    indices: vec![0, 1, 2, 2, 3, 0],
                    normals: vec![
                        [0.0, -1.0, 0.0],
                        [0.0, -1.0, 0.0],
                        [0.0, -1.0, 0.0],
                        [0.0, -1.0, 0.0],
                    ],
                    colors: vec![
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    ],
                    uvs: vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
                },
                Face {
                    direction: FaceDirection::Front,
                    vertices: vec![
                        [0., 0., 0.], // A 00 Front [0]
                        [1., 0., 0.], // B 01 Front [1]
                        [1., 1., 0.], // C 02 Front [2]
                        [0., 1., 0.],
                    ],
                    indices: vec![0, 3, 2, 2, 1, 0],
                    normals: vec![
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0, -1.0],
                        [0.0, 0.0, -1.0],
                    ],
                    colors: vec![
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    ],
                    uvs: vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
                },
                Face {
                    direction: FaceDirection::Back,
                    vertices: vec![[0., 0., 1.], [1., 0., 1.], [1., 1., 1.], [0., 1., 1.]],
                    indices: vec![0, 1, 2, 2, 3, 0],
                    normals: vec![
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                        [0.0, 0.0, 1.0],
                    ],
                    colors: vec![
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    ],
                    uvs: vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
                },
                Face {
                    direction: FaceDirection::Left,
                    vertices: vec![[0., 1., 0.], [0., 0., 0.], [0., 0., 1.], [0., 1., 1.]],
                    indices: vec![3, 0, 1, 1, 2, 3],
                    normals: vec![
                        [-1.0, 0.0, 0.0],
                        [-1.0, 0.0, 0.0],
                        [-1.0, 0.0, 0.0],
                        [-1.0, 0.0, 0.0],
                    ],
                    colors: vec![
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    ],
                    uvs: vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
                },
                Face {
                    direction: FaceDirection::Right,
                    vertices: vec![[1., 0., 0.], [1., 1., 0.], [1., 1., 1.], [1., 0., 1.]],
                    indices: vec![0, 1, 2, 2, 3, 0],
                    normals: vec![
                        [1.0, 0.0, 0.0],
                        [1.0, 0.0, 0.0],
                        [1.0, 0.0, 0.0],
                        [1.0, 0.0, 0.0],
                    ],
                    colors: vec![
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                        [1.0, 1.0, 1.0, 1.0],
                    ],
                    uvs: vec![[0., 0.], [1., 0.], [1., 1.], [0., 1.]],
                },
            ],
        }
    }
}
