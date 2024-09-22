mod controller;
mod spawn;

pub use controller::*;
pub use spawn::*;
use crate::input::keyboard::*;
use crate::camera::CameraController;
use bevy::prelude::*;


#[derive(Component)]
pub struct Player {
    pub vertical_velocity: f32,
    pub on_ground: bool,
    pub view_mode: ViewMode,
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
        }
    }

    pub fn toggle_view_mode(&mut self) {
        self.view_mode = match self.view_mode {
            ViewMode::FirstPerson => ViewMode::ThirdPerson,
            ViewMode::ThirdPerson => ViewMode::FirstPerson,
        };
    }
}
