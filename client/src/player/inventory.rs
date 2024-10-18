use std::collections::HashMap;

use crate::constants::{MAX_INVENTORY_SLOTS, MAX_ITEM_STACK};
use crate::ui::inventory::items;
use crate::ui::inventory::items::Item;
use bevy::prelude::*;
use shared::world::ItemId;
use shared::world::RegistryId;

#[derive(Debug, Resource, Clone)]
pub struct Inventory {
    pub inner: HashMap<RegistryId, items::Item>,
}

impl Inventory {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    // Ajoute un item à l'inventaire du joueur
    pub fn add_item_to_inventory(&mut self, item_id: ItemId, mut nb: u32) {
        for i in 0..MAX_INVENTORY_SLOTS {
            let item_option = self.inner.get(&i);

            if item_option.is_some() {
                let existing_item = item_option.expect("Error : empty item");
                // If not item of right type or stack already full : pass
                if existing_item.id != item_id || existing_item.nb >= MAX_ITEM_STACK {
                    continue;
                }

                nb += existing_item.nb;
            }

            let inserted_items = if nb >= MAX_ITEM_STACK {
                MAX_ITEM_STACK
            } else {
                nb
            };
            nb -= inserted_items;

            // Push inserted items in right inventory slot
            self.inner.insert(
                i,
                items::Item {
                    id: item_id,
                    nb: inserted_items,
                },
            );

            // If no more items to add, end loop
            if nb == 0 {
                break;
            }
        }

        // Problem : if inventory full, items disappear
    }

    /// Add items to stack at specified position\
    /// Stacks cannot exceed MAX_ITEM_STACK number of items\
    /// Returns number of items really added to the stack
    pub fn add_item_to_stack(&mut self, item_id: ItemId, stack: u32, mut nb: u32) -> u32 {
        let item_option = self.inner.get(&stack);
        let mut new_item = Item { id: item_id, nb };

        if let Some(item) = item_option {
            if nb + item.nb > MAX_ITEM_STACK {
                nb = MAX_ITEM_STACK - item.nb;
            }
            new_item.nb = nb + item.nb;
        }
        self.inner.insert(stack, new_item);
        nb
    }

    /// Removes items from stack at specified position\
    /// Stacks cannot have < 0 number of items\
    /// Returns number of items really removed from the stack
    pub fn remove_item_from_stack(&mut self, stack: u32, mut nb: u32) -> u32 {
        let item_option = self.inner.get(&stack);

        if let Some(&item) = item_option {
            if nb >= item.nb {
                nb = item.nb;
                self.inner.remove(&stack);
            } else {
                self.inner.insert(
                    stack,
                    Item {
                        id: item.id,
                        nb: item.nb - nb,
                    },
                );
            }
            return nb;
        }
        0
    }
}

// ! ------- UNUSED CODE ------------
// Renvoie l'emplacement d'un stack de l'item donné dans l'inventaire, ou None s'il n'existe pas
// pub fn find_item_in_inventory(player: &Player, item_id: items::ItemsType) -> Option<Item> {
//     for item in self.inner.values() {
//         if item.id == item_id {
//             return Some(*item);
//         }
//     }
//     None
// }
