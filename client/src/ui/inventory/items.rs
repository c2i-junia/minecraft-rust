use rand::Rng;
use shared::world::{BlockData, ItemData, ItemType, Registry, RegistryId};

use crate::constants::MAX_ITEM_STACK;
use crate::ui::inventory::FloatingStack;

pub type Item = shared::world::Item;

pub fn item_from_block(block: &RegistryId, r_blocks: &Registry<BlockData>) -> Option<RegistryId> {
    let pool = r_blocks.get(block).unwrap().drops.clone();
    let total = pool
        .clone()
        .into_iter()
        .reduce(|a, b| (a.0 + b.0, a.1))
        .unwrap()
        .0;
    let mut nb = rand::thread_rng().gen_range(0..total);

    // Choose drop item
    for item in pool {
        if nb < item.0 {
            return Some(item.1);
        } else {
            nb -= item.0;
        }
    }
    None
}

pub fn block_from_item(item: &RegistryId, r_items: &Registry<ItemData>) -> Option<RegistryId> {
    if let ItemType::Block(block) = r_items.get(item).unwrap().kind {
        Some(block)
    } else {
        None
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
    item_type: RegistryId,
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
