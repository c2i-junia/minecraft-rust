use crate::{constants::INTERACTION_DISTANCE, world::WorldMap, Block, BlockRaycastSet, Player};
use bevy::{math::NormedVectorSpace, prelude::*};
use bevy_mod_raycast::prelude::RaycastSource;

#[derive(Component)]
pub struct BlockText;

// Updates UI to tell which block the player is looking at (or none if no block is within INTERACTION_DISTANCE)
pub fn block_text_update_system(
    player: Query<&Transform, With<Player>>,
    world_map: Res<WorldMap>,
    mut query: Query<&mut Text, With<BlockText>>,
    raycast_source: Query<&RaycastSource<BlockRaycastSet>>, // raycast (to get current "selected" block)
) {

    let raycast_source = raycast_source.single();

    let mut col = Color::srgb(0.2, 0.2, 0.2);
    let mut txt = "<none>".to_string();

    if let Some((entity, intersection)) = raycast_source.intersections().first() {
        // Check if block is close enough to the player
        if (intersection.position() - player.single().translation).norm() < INTERACTION_DISTANCE {
            let block_type = world_map.get_block_wrapper_by_entity(*entity).unwrap().kind;
            col = match block_type {
                Block::Bedrock => Color::srgb(0.4, 0.4, 0.4),
                Block::Dirt => Color::Srgba(Srgba::hex("69512E").unwrap()),
                Block::Grass => Color::Srgba(Srgba::hex("7CFC00").unwrap()),
                Block::Stone => Color::Srgba(Srgba::hex("888C8D").unwrap())
            };
            txt = format!("{:?}", block_type);
        }
    }

    for mut text in query.iter_mut() {
        text.sections[1].style.color = col;
        text.sections[1].value = txt.clone();
    }
}
