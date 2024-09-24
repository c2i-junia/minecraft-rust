mod controller;
mod spawn;
mod inventory;

use bevy::prelude::*;
pub use controller::*;
pub use spawn::*;

#[derive(Component)]
pub struct Player {
    pub vertical_velocity: f32,
    pub on_ground: bool,
    pub view_mode: ViewMode,
    pub is_chunk_debug_mode_enabled: bool,
    pub inventory : Vec<Items>
}

pub struct Items {
    id : i32,
    nb : i32
}

#[derive(Debug, PartialEq)]
pub enum ViewMode {
    FirstPerson,
    ThirdPerson,
}

impl Player {
    pub fn new() -> Self {
        Self {
            vertical_velocity: 0.0,
            on_ground: true,
            view_mode: ViewMode::FirstPerson,
            is_chunk_debug_mode_enabled: true,
            inventory : vec![]
        }
    }

    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::FirstPerson => ViewMode::ThirdPerson,
            ViewMode::ThirdPerson => ViewMode::FirstPerson,
        };
    }

    pub fn toggle_chunk_debug_mode(&mut self) {
        self.is_chunk_debug_mode_enabled = !self.is_chunk_debug_mode_enabled;
    }
}
