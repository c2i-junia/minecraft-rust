use crate::Block;

#[derive(Debug, Clone)]
pub struct Item {
    pub id: ItemsType,
    pub nb: u32,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum ItemsType {
    Grass,
    Dirt,
    Stone,
    Bedrock,
}

pub fn item_from_block(block: Block) -> Option<ItemsType> {
    match block {
        Block::Bedrock => Some(ItemsType::Bedrock),
        Block::Dirt | Block::Grass => Some(ItemsType::Dirt),
        Block::Stone => Some(ItemsType::Stone), // _ => None
    }
}

pub fn block_from_item(item: ItemsType) -> Option<Block> {
    match item {
        ItemsType::Bedrock => Some(Block::Bedrock),
        ItemsType::Dirt => Some(Block::Dirt),
        ItemsType::Grass => Some(Block::Grass),
        ItemsType::Stone => Some(Block::Stone),
        // _ => None
    }
}
