use crate::{
    camera::BlockRaycastSet,
    constants::{CUBE_SIZE, INTERACTION_DISTANCE},
    player::Player,
    world::WorldMap,
};
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

    let mut col = Color::srgb(1., 1., 1.);
    let mut txt = "<none>".to_string();

    if let Some((_entity, intersection)) = raycast_source.intersections().first() {
        // Check if block is close enough to the player
        if (intersection.position() - player.single().translation).norm() < INTERACTION_DISTANCE {
            let block_pos = intersection.position() - intersection.normal() * (CUBE_SIZE / 2.);
            let vec = IVec3::new(
                block_pos.x.round() as i32,
                block_pos.y.round() as i32,
                block_pos.z.round() as i32,
            );
            let block = world_map.get_block_by_coordinates(&vec);
            let block = match block {
                Some(v) => v,
                None => return,
            };
            col = Color::WHITE;
            txt = format!(
                "{:?} | pos = {}",
                block,
                intersection.position().xyz().round()
            );
        }
    }

    for mut text in query.iter_mut() {
        text.sections[1].style.color = col;
        text.sections[1].value = txt.clone();
    }
}
