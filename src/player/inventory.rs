use crate::player;
use crate::player::Player;
use bevy::prelude::*;

// Ajoute un item à l'inventaire du joueur
pub fn add_item_to_inventory(player: &mut Query<&mut Player>, item_id: i32, nb: i32) {
    let mut player = player.single_mut();
    for item in player.inventory.iter_mut() {
        if item.id == item_id {
            item.nb += nb;
            return;
        }
    }
    player.inventory.push(player::Items {
        id: item_id,
        nb: nb,
    });
}

// Retire un item de l'inventaire du joueur
pub fn remove_item_from_inventory(player: &mut Query<&mut Player>, item_id: i32, nb: i32) {
    let mut player = player.single_mut();
    for item in player.inventory.iter_mut() {
        if item.id == item_id {
            item.nb -= nb;
            if item.nb <= 0 {
                player.inventory.retain(|x| x.id != item_id);
            }
            return;
        }
    }
}

// Retourne le nombre d'items dans l'inventaire du joueur
// pub fn get_item_count(player: &Player, item_id: i32) -> i32 {
//     for item in player.inventory.iter() {
//         if item.id == item_id {
//             return item.nb;
//         }
//     }
//     return 0;
// }

// Retourne true si le joueur possède l'item
pub fn has_item(player: &mut Query<&mut Player>, item_id: i32) -> bool {
    let player = player.single_mut();
    for item in player.inventory.iter() {
        if item.id == item_id {
            return true;
        }
    }
    return false;
}
