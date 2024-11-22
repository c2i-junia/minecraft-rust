use crate::GameState;
use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Player {
    pub vertical_velocity: f32,
    pub on_ground: bool,
    // pub view_mode: ViewMode,
    // pub is_chunk_debug_mode_enabled: bool,
    pub is_flying: bool,
    // pub inventory: HashMap<RegistryId, items::Item>,
    pub height: f32,
    pub width: f32,
}

#[derive(Debug, PartialEq, Clone, Copy, Resource)]
pub enum ViewMode {
    FirstPerson,
    ThirdPerson,
}

impl ViewMode {
    pub fn toggle(&mut self) {
        *self = match *self {
            ViewMode::FirstPerson => ViewMode::ThirdPerson,
            ViewMode::ThirdPerson => ViewMode::FirstPerson,
        };
    }
}

impl Player {
    pub fn new() -> Self {
        Self {
            vertical_velocity: 0.0,
            on_ground: true,
            is_flying: false,
            height: 1.8,
            width: 0.8,
        }
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

    let spawn_coords = Vec3::new(7.5, 80.0, 7.5);

    commands
        .spawn((
            StateScoped(GameState::Game),
            PbrBundle {
                mesh: meshes.add(Mesh::from(Cuboid::new(
                    player.width,
                    player.height,
                    player.width,
                ))),
                material: materials.add(Color::srgba(1.0, 0.0, 0.0, 0.0)),
                transform: Transform::from_translation(spawn_coords),
                ..Default::default()
            },
        ))
        .insert(player);
}
