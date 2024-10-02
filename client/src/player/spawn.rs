use crate::ui::items;
use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Component, Clone)]
pub struct Player {
    pub vertical_velocity: f32,
    pub on_ground: bool,
    pub view_mode: ViewMode,
    pub is_chunk_debug_mode_enabled: bool,
    pub is_flying: bool,
    pub inventory: HashMap<u32, items::Item>,
    pub height: f32,
    pub width: f32,
}

#[derive(Debug, PartialEq, Clone, Copy)]
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
            is_flying: false,
            inventory: HashMap::new(), // No items in the inventory at the beginning
            height: 1.8,
            width: 0.8,
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

    pub fn toggle_fly_mode(&mut self) {
        self.is_flying = !self.is_flying;
        self.vertical_velocity = 0.0; // Réinitialisation de la vélocité
    }
}

pub fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = Player::new();

    let spawn_coords = Vec3::new(7.5, 45.0, 7.5);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(
                player.width,
                player.height,
                player.width,
            ))),
            material: materials.add(Color::srgba(1.0, 0.0, 0.0, 0.0)),
            transform: Transform::from_translation(spawn_coords),
            ..Default::default()
        })
        .insert(player);
}
