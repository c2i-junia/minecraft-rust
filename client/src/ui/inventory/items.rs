use crate::constants::MAX_ITEM_STACK;
use crate::ui::inventory::FloatingStack;
use crate::world::Block;

pub type Item = shared::world::Item;
pub type ItemId = shared::world::ItemId;

pub fn item_from_block(block: Block) -> Option<ItemId> {
    match block {
        Block::Bedrock => Some(ItemId::Bedrock),
        Block::Dirt | Block::Grass => Some(ItemId::Dirt),
        Block::Stone => Some(ItemId::Stone), // _ => None
    }
}

pub fn block_from_item(item: ItemId) -> Option<Block> {
    match item {
        ItemId::Bedrock => Some(Block::Bedrock),
        ItemId::Dirt => Some(Block::Dirt),
        ItemId::Grass => Some(Block::Grass),
        ItemId::Stone => Some(Block::Stone),
        // _ => None
    }
}

/// Removes `nb` items from the floating stack\
/// Cannot go lower than `0` items\
/// Returns number of items _actually_ removed
pub fn remove_item_floating_stack(floating_stack: &mut FloatingStack, nb: u32) -> u32 {
    if let Some(mut item) = floating_stack.items {
        if nb >= item.nb {
            floating_stack.items = None;
            return item.nb;
        }
        item.nb -= nb;
        floating_stack.items = Some(item);
        return nb;
    }
    0
}

/// Adds `nb` items to the floating stack\
/// Cannot go higher than `MAX_ITEM_STACK` items\
/// Parameter `item_type` will **ONLY BE USED** if no items are present in the floating stack\
/// Returns number of items _actually_ added
pub fn add_item_floating_stack(
    floating_stack: &mut FloatingStack,
    mut nb: u32,
    item_type: ItemId,
) -> u32 {
    if nb == 0 {
        0
    } else if let Some(mut item) = floating_stack.items {
        if nb + item.nb > MAX_ITEM_STACK {
            nb = MAX_ITEM_STACK - item.nb;
        }
        item.nb += nb;
        nb
    } else {
        if nb > MAX_ITEM_STACK {
            nb = MAX_ITEM_STACK;
        }
        floating_stack.items = Some(Item { id: item_type, nb });
        nb
    }
}
