pub mod data;
pub mod materials;
mod meshing;
pub mod render;
mod render_distance;
pub mod time;
mod voxel;

pub use data::*;
// pub use load_from_file::*;
pub use materials::*;
pub use render::*;
pub use render_distance::*;

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct FirstChunkReceived(pub bool);
