use crate::constants::CHUNK_SIZE;
use crate::player::Player;
use bevy::prelude::*;
use noise::{NoiseFn, Perlin};
use rand::Rng;
use shared::world::{
    block_to_chunk_coord, to_global_pos, BlockData, BlockType, ItemData, Registry, SIX_OFFSETS
};
use std::collections::HashSet;

use crate::{world::*, GameState, LoadWorldEvent};

use super::RenderDistance;

fn generate_chunk(
    chunk_pos: IVec3,
    seed: u32,
    world_map: &mut WorldMap,
    ev_render: &mut EventWriter<WorldRenderRequestUpdateEvent>,
    registry: &Registry<BlockData>,
) {
    //println!("gen chunk {}", chunk_pos);
    let perlin = Perlin::new(seed);

    let scale = 0.1;
    let max_perlin_height_variation = 5.0;
    let base_height = 32; // should be 64

    const WORLD_MIN_Y: i32 = 0;

    let cx = chunk_pos.x;
    let cz = chunk_pos.z;

    // Boucle pour générer les blocs avec variation de hauteur
    for i in 0..CHUNK_SIZE {
        for j in 0..CHUNK_SIZE {
            let x = CHUNK_SIZE * cx + i;
            let z = CHUNK_SIZE * cz + j;

            // Générer une hauteur en utilisant le bruit de Perlin
            let perlin_height = (perlin.get([x as f64 * scale, z as f64 * scale]) - 0.5)
                * max_perlin_height_variation;

            // Ajouter un offset de 64 blocs pour centrer la hauteur autour de y = 64
            let terrain_height = base_height + perlin_height.round() as i32;

            // Générer des blocs à partir de la couche 0 (bedrock) jusqu'à la hauteur générée
            for y in WORLD_MIN_Y..=terrain_height {
                let block = if y == 0 {
                    BlockType::Bedrock.get_name()
                } else if y < terrain_height - 2 {
                    BlockType::Stone.get_name()
                } else if y < terrain_height {
                    BlockType::Dirt.get_name()
                } else {
                    BlockType::Grass.get_name()
                };

                // Get block id from name, then set it
                world_map.set_block(
                    &IVec3::new(x, y, z),
                    *registry.get_id(&block).unwrap(),
                );

                // Incrémenter le compteur de blocs
                world_map.total_blocks_count += 1;
            }
        }
    }

    world_map.total_chunks_count += 1;
    for y in 0..=3 {
        let mut pos = chunk_pos;
        pos.y = y;
        ev_render.send(WorldRenderRequestUpdateEvent::ChunkToReload(pos));
    }
    // println!("sending event for {}", chunk_pos);
}

pub fn setup_world(
    mut commands: Commands,
    mut world_map: ResMut<WorldMap>,
    mut ev_render: EventWriter<WorldRenderRequestUpdateEvent>,
    mut ev_load: EventReader<LoadWorldEvent>,
    mut player_query: Query<(&mut Transform, &mut Player)>,
    r_blocks: Res<Registry<BlockData>>,
    r_items: Res<Registry<ItemData>>,
) {
    let mut world_name = "default";
    // Get loaded world name
    for ev in ev_load.read() {
        world_name = &ev.world_name;
    }

    world_map.name = world_name.into();

    let (mut transform, mut player) = player_query.single_mut();

    // Charger la graine depuis le fichier `{world_name}_seed.ron`
    let seed = match load_world_seed(world_name) {
        Ok(seed) => {
            println!("Loaded existing world seed from {}_seed.ron", world_name);
            seed.0
        }
        Err(_) => {
            // Si la graine n'est pas trouvée, en générer une nouvelle
            let seed = rand::thread_rng().gen::<u32>();
            println!("Generated random seed: {}", seed);
            seed
        }
    };

    commands.insert_resource(WorldSeed(seed));

    // Charger la carte du monde depuis le fichier `{world_name}_save.ron`
    if let Ok(loaded_world) = load_world_map(
        world_name,
        &mut player,
        &mut transform.translation,
        &r_items,
        &r_blocks,
    ) {
        *world_map = loaded_world;
        println!("Loaded existing world from {}_save.ron", world_name);

        world_map.name = world_name.into();

        // we need to recreate the entities because their are not
        // saved in the world_save file
        for (chunk_pos, chunk) in world_map.map.iter_mut() {
            if chunk.entity.is_none() {
                let new_entity = commands
                    .spawn((
                        StateScoped(GameState::Game),
                        Transform::from_xyz(
                            (chunk_pos.x * CHUNK_SIZE) as f32,
                            (chunk_pos.y * CHUNK_SIZE) as f32,
                            (chunk_pos.z * CHUNK_SIZE) as f32,
                        ),
                        GlobalTransform::default(),
                    ))
                    .id();
                chunk.entity = Some(new_entity);
            }

            // now that the entities are loaded, we need to send events to
            // update the rendering
            for x in -1..=1 {
                for y in 0..=8 {
                    for z in -1..=1 {
                        ev_render.send(WorldRenderRequestUpdateEvent::ChunkToReload(IVec3::new(
                            x, y, z,
                        )));
                    }
                }
            }
        }
    } else {
        // Si le chargement échoue, on génère un nouveau monde
        println!("Generating a new world with seed: {}", seed);

        for x in -1..=1 {
            for y in 0..=8 {
                for z in -1..=1 {
                    generate_chunk(
                        IVec3::new(x, y, z),
                        seed,
                        &mut world_map,
                        &mut ev_render,
                        &r_blocks,
                    );
                }
            }
        }
    }
}

pub fn load_chunk_around_player(
    player_position: Vec3,
    world_map: &mut WorldMap,
    seed: u32,
    ev_render: &mut EventWriter<WorldRenderRequestUpdateEvent>,
    render_distance: Res<RenderDistance>,
    r_blocks: &Registry<BlockData>,
) {
    let player_chunk = IVec3::new(
        block_to_chunk_coord(player_position.x as i32),
        0,
        block_to_chunk_coord(player_position.z as i32),
    );

    let r = render_distance.distance as i32;

    for x in -r..=r {
        for z in -r..=r {
            let chunk_pos = IVec3::new(player_chunk.x + x, 0, player_chunk.z + z);
            {
                let chunk = world_map.map.get(&chunk_pos);
                if chunk.is_some() {
                    continue;
                }
                // Doing these scoping shenanigans to release the Mutex at the end of the scope
                // because generate_chunk requires a Mutex lock as well
            }
            generate_chunk(chunk_pos, seed, world_map, ev_render, r_blocks);
        }
    }
}

#[derive(Event, Debug, Copy, Clone, Hash, Eq, PartialEq)]
pub enum WorldRenderRequestUpdateEvent {
    ChunkToReload(IVec3),
    BlockToReload(IVec3),
}

pub(crate) fn is_block_surrounded(
    world_map: &WorldMap,
    chunk_pos: &IVec3,
    local_block_pos: &IVec3,
) -> bool {
    let global_block_pos = to_global_pos(chunk_pos, local_block_pos);

    for offset in &SIX_OFFSETS {
        let neighbor_pos = global_block_pos + *offset;

        // Check if the block exists at the neighboring position
        if world_map.get_block_by_coordinates(&neighbor_pos).is_none() {
            return false;
        }
    }

    true
}

#[derive(Default, Debug)]
pub struct QueuedEvents {
    pub events: HashSet<WorldRenderRequestUpdateEvent>,
}
