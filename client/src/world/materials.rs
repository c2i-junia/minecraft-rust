use crate::constants::{BASE_ROUGHNESS, BASE_SPECULAR_HIGHLIGHT};
use crate::game::PreLoadingCompletion;
use crate::world::GlobalMaterial;
use crate::TexturePath;
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, Face, TextureDimension, TextureFormat};
use shared::world::{get_game_folder, BlockId, GameElementId, ItemId};
use shared::{AssetsFolderPath, GameFolderPath};
use std::collections::HashMap;
use std::fs;
use std::marker::PhantomData;

use super::meshing::UvCoords;

#[derive(Default, Resource)]
pub struct AtlasWrapper {
    pub uvs: HashMap<String, UvCoords>,
    pub material: Option<Handle<StandardMaterial>>,
    pub texture: Option<Handle<Image>>,
}

#[derive(Resource, Default)]
pub struct MaterialResource {
    pub global_materials: HashMap<GlobalMaterial, Handle<StandardMaterial>>,
    pub items: AtlasWrapper,
    pub blocks: AtlasWrapper,
}

#[derive(Resource)]
pub struct AtlasHandles<T> {
    pub handles: Vec<(Handle<Image>, String)>,
    pub loaded: bool,
    /// Phantom to allow multiple instances of the struct
    _d: PhantomData<T>,
}

impl<T> Default for AtlasHandles<T> {
    fn default() -> Self {
        Self {
            handles: Vec::new(),
            loaded: false,
            _d: PhantomData {},
        }
    }
}

pub fn setup_materials(
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_resource: ResMut<MaterialResource>,
    mut block_atlas_handles: ResMut<AtlasHandles<BlockId>>,
    mut item_atlas_handles: ResMut<AtlasHandles<ItemId>>,
    texture_path: Res<TexturePath>,
    game_folder_path: Res<GameFolderPath>,
    assets_folder_path: Res<AssetsFolderPath>,
) {
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

    // material_resource
    //     .block_materials
    //     .insert(BlockId::Dirt, dirt_material);
    // material_resource
    //     .block_materials
    //     .insert(BlockId::Grass, grass_material);
    // material_resource
    //     .block_materials
    //     .insert(BlockId::Stone, stone_material);
    // material_resource
    //     .block_materials
    //     .insert(BlockId::Bedrock, bedrock_material);

    let actual_assets_folder_path = assets_folder_path
        .0
        .clone()
        .unwrap_or(format!("{}/data/", game_folder_path.0.clone()).into());

    material_resource
        .global_materials
        .insert(GlobalMaterial::Sun, sun_material);
    material_resource
        .global_materials
        .insert(GlobalMaterial::Moon, moon_material);

    // material_resource.item_textures.insert(
    //     ItemId::Grass,
    //     asset_server.load(&(TEXTURE_PATH.to_owned() + "grass.png")),
    // );
    // material_resource.item_textures.insert(
    //     ItemId::Dirt,
    //     asset_server.load(&(TEXTURE_PATH.to_owned() + "dirt.png")),
    // );
    // material_resource.item_textures.insert(
    //     ItemId::Stone,
    //     asset_server.load(&(TEXTURE_PATH.to_owned() + "stone.png")),
    // );
    // material_resource.item_textures.insert(
    //     ItemId::Bedrock,
    //     asset_server.load(&(TEXTURE_PATH.to_owned() + "bedrock.png")),
    // );

    // let image_paths = ["moss.png", "dirt.png", "stone.png", "bedrock.png"];

    // Load the images asynchronously
    // let handles: Vec<Handle<Image>> = image_paths
    //     .iter()
    //     .map(|filename| asset_server.load(&(TEXTURE_PATH.to_owned() + filename)))
    //     .collect();

    // Load images of all blocks defined in the enum

    let blocks_path = get_game_folder(Some(&game_folder_path))
        .join(actual_assets_folder_path.clone())
        .join(&texture_path.path)
        .join("blocks/");
    let items_path = get_game_folder(Some(&game_folder_path))
        .join(actual_assets_folder_path)
        .join(&texture_path.path)
        .join("items/");

    if let Ok(dir) = fs::read_dir(blocks_path.clone()) {
        block_atlas_handles.handles = dir
            .map(|file| {
                let binding = file.unwrap().path();
                let filename = binding.file_stem().unwrap().to_str().unwrap();
                (
                    asset_server.load(
                        blocks_path
                            .join(filename)
                            .with_extension("png")
                            .to_string_lossy()
                            .into_owned(),
                    ),
                    filename.to_owned(),
                )
            })
            .collect();
        info!("Block textures loaded");
    } else {
        warn!(
            "Block textures could not be loaded. This could crash the game : {:?}",
            blocks_path.display()
        );
    }

    if let Ok(dir) = fs::read_dir(items_path.clone()) {
        item_atlas_handles.handles = dir
            .map(|file| {
                let binding = file.unwrap().path();
                let filename = binding.file_stem().unwrap().to_str().unwrap();
                (
                    asset_server.load(
                        items_path
                            .join(filename)
                            .with_extension("png")
                            .to_string_lossy()
                            .into_owned(),
                    ),
                    filename.to_owned(),
                )
            })
            .collect();
        info!("Block textures loaded");
    } else {
        warn!(
            "Block textures could not be loaded. This could crash the game : {:?}",
            items_path.display()
        );
    }
}

pub fn create_all_atlases(
    mut atlases: (ResMut<AtlasHandles<BlockId>>, ResMut<AtlasHandles<ItemId>>),
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut material_resource: ResMut<MaterialResource>,
    mut loading: ResMut<PreLoadingCompletion>,
) {
    build_atlas(
        &mut atlases.0,
        &mut images,
        &mut materials,
        &mut material_resource.blocks,
    );

    build_atlas(
        &mut atlases.1,
        &mut images,
        &mut materials,
        &mut material_resource.items,
    );

    if material_resource.items.texture.is_some() && material_resource.items.texture.is_some() {
        loading.textures_loaded = true;
    }
}

fn build_atlas<T: GameElementId>(
    atlas_handles: &mut AtlasHandles<T>,
    images: &mut ResMut<Assets<Image>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    atlas: &mut AtlasWrapper,
) {
    if atlas_handles.loaded {
        // Blocks if this atlas is loaded but game setup phase is not done yet
        return;
    }
    // Check if all images have been loaded
    let loaded_images: Vec<(&Image, &String)> = atlas_handles
        .handles
        .iter()
        .filter_map(|(handle, block)| images.get(handle).map(|image| (image, block)))
        .collect();

    if loaded_images.len() != atlas_handles.handles.len() {
        // Not all images are loaded yet
        return;
    }

    println!("Debug handle : {:?}", atlas_handles.handles[0]);

    // Assuming each image is 16x16 and there are `n` images
    let image_count: usize = loaded_images.len();
    let atlas_width: u32 = (image_count * 16) as u32;
    let atlas_height: u32 = 16;
    let mut atlas_data: Vec<u8> = vec![0u8; (atlas_width * atlas_height * 4) as usize]; // RGBA format

    // Copy the pixels of each image into the correct position in the atlas
    for (i, (image, id)) in loaded_images.iter().enumerate() {
        let offset_x = i * 16;
        atlas.uvs.insert(
            (*id).clone(),
            UvCoords::new(
                offset_x as f32 / atlas_width as f32,
                (offset_x + 16) as f32 / atlas_width as f32,
                0.,
                1.,
            ),
        );
        for y in 0..16 {
            for x in 0..16 {
                let src_index = (y * 16 + x) * 4;
                let dest_index = ((y * atlas_width as usize) + (x + offset_x)) * 4;
                atlas_data[dest_index..dest_index + 4]
                    .copy_from_slice(&image.data[src_index..src_index + 4]);
            }
        }
    }

    // Create the atlas texture
    let atlas_texture = Image::new_fill(
        Extent3d {
            width: atlas_width,
            height: atlas_height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &atlas_data,
        TextureFormat::Rgba8UnormSrgb,
        default(),
    );

    // Add the atlas texture as an asset
    let atlas_handle = images.add(atlas_texture);

    atlas.texture = Some(atlas_handle.clone_weak());

    let atlas_material = materials.add(StandardMaterial {
        base_color_texture: Some(atlas_handle),
        perceptual_roughness: BASE_ROUGHNESS,
        reflectance: BASE_SPECULAR_HIGHLIGHT,
        alpha_mode: AlphaMode::Mask(0.5),
        ..default()
    });

    atlas.material = Some(atlas_material);

    atlas_handles.loaded = true;
}
