use bevy::prelude::{Component, Query, Visibility, With};

use crate::Player;

/// All UI dialogs toggling mouse visibility MUST use this in their bundle list\
/// They must also possess the `visibility` attribute\
/// Basically used to detect if multiple dialogs are open at once\
/// For example, mouse visibility : must stay visible as long as at least one dialog is active\
/// When the last active dialog is hidden, the mouse too\
#[derive(Component)]
pub struct UiDialog;

#[derive(PartialEq, Eq)]
pub enum UIMode {
    Opened,
    Closed,
}

pub fn set_ui_mode(mut player: Query<&mut Player>, visibility: Query<&Visibility, With<UiDialog>>) {
    let mut player = player.single_mut();
    for vis in visibility.iter() {
        if vis == Visibility::Visible {
            player.ui_mode = UIMode::Opened;
            return;
        }
    }
    player.ui_mode = UIMode::Closed;
}

mod display;
mod inv;
pub mod items;
mod setup;

pub use display::*;
pub use inv::*;
use items::*;
pub use setup::*;
