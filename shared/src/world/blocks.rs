use std::{collections::HashMap, mem::transmute};

use super::{GameElementId, ItemId};
use rand::Rng;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, serde::Serialize, serde::Deserialize, Hash,
    Default
)]
#[repr(usize)]
pub enum BlockId {
    #[default]
    Dirt,
    // ! ----- LEAVE DIRT FIRST ----- !
    Grass,
    Stone,
    // ! ----- LEAVE BEDROCK LAST ----- !
    Bedrock,
}

pub enum BlockTags {
    Solid,
    Stone,
}

impl BlockId {
    pub fn is_biome_colored() -> bool {
        false
    }

    pub fn get_break_time(&self) -> f32 {
        match *self {
            Self::Bedrock => -1.,
            _ => 5.,
        }
    }

    pub fn get_uvs(&self) -> [f32; 4] {
        match *self {
            BlockId::Bedrock => [0.75, 1.0, 0.0, 1.0],
            BlockId::Dirt => [0.25, 0.5, 0.0, 1.0],
            BlockId::Grass => [0.0, 0.25, 0.0, 1.0],
            BlockId::Stone => [0.5, 0.75, 0.0, 1.0],
        }
    }

    pub fn get_drops(&self, nb_drops: u32) -> HashMap<ItemId, u32> {
        let mut drops = HashMap::new();
        let table = self.get_drop_table();

        // Choose drop items
        for _ in 0..nb_drops {
            let mut nb = rand::thread_rng().gen_range(0.0..100.0);
            for item in table.iter() {
                if nb < item.0 {
                    drops.insert(item.1, 1);
                } else {
                    nb -= item.0;
                }
            }
        }
        drops
    }

    pub fn get_drop_table(&self) -> Vec<(f32, ItemId)> {
        match *self {
            BlockId::Dirt | BlockId::Grass => vec![(100., ItemId::Dirt)],
            BlockId::Stone => vec![(100., ItemId::Stone)],
            _ => vec![],
        }
    }

    pub fn get_tags(&self) -> Vec<BlockTags> {
        match *self {
            BlockId::Stone => vec![BlockTags::Stone, BlockTags::Solid],
            _ => vec![BlockTags::Solid],
        }
    }
}

impl GameElementId for BlockId {
    fn iterate_enum() -> impl Iterator<Item = BlockId> {
        // Unsafe code needed for `transmute` function
        // Transmute function needed to cast from `usize` to `BlockId`
        // Still safe, because `BlockId` enum only contains numerical enum variants
        unsafe { ((Self::Dirt as usize)..=(Self::Bedrock as usize)).map(|num| transmute(num)) }
    }
}