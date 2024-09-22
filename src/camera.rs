use crate::{player, Player};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use bevy_mod_raycast::prelude::*;

#[derive(TypePath)]
pub struct BlockRaycastSet;

#[derive(Component)]
pub struct CameraController {
    distance: f32,
    angle_x: f32,
    angle_y: f32,
    mouse_sensitivity: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            distance: 10.0,
            angle_x: 0.0,
            angle_y: 20.0f32.to_radians(),
            mouse_sensitivity: 0.003,
        }
    }
}



pub fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 5.0, 10.0))
                .looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
            ..Default::default()
        })
        .insert(CameraController::default())  // Ajoute le CameraController
        .insert({
            let mut raycast_source = RaycastSource::<BlockRaycastSet>::default();  // Initialisation par défaut
            raycast_source.cast_method = RaycastMethod::Transform;  // Utilise la transformation de la caméra pour lancer le rayon
            raycast_source  // Retourne l'objet 
        });
}

pub fn spawn_reticle(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(5.0),  // Largeur du réticule
            height: Val::Px(5.0), // Hauteur du réticule
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Auto,
                bottom: Val::Auto,
            },
            position_type: PositionType::Absolute,
            ..Default::default()
        },
        background_color: Color::WHITE.into(), // Couleur du réticule
        ..Default::default()
    });
}

// System to hide and lock the cursor
pub fn cursor_grab_system(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.cursor.visible = false;
    window.cursor.grab_mode = CursorGrabMode::Locked;
}

// System to control the camera based on mouse movement
pub fn camera_control_system(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut camera_query: Query<
        (&mut Transform, &mut CameraController),
        (With<Camera>, Without<Player>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Camera>)>,
    player: Query<&Player>,
) {
    let window = windows.single();

    // Skip if the window is not focused
    if !window.focused {
        return;
    }

    let mut delta = Vec2::ZERO;

    // Accumulate mouse motion events
    for event in mouse_motion_events.read() {
        delta += event.delta;
    }

    player.iter().for_each(|player| {
        if player.view_mode == player::ViewMode::FirstPerson {
            for (_, mut controller) in camera_query.iter_mut() {
                controller.distance = 1.0;
            }
        } else if player.view_mode == player::ViewMode::ThirdPerson {
            for (_, mut controller) in camera_query.iter_mut() {
                controller.distance = 10.0;
            }
        }
    });

    for (mut transform, mut controller) in camera_query.iter_mut() {
        controller.angle_x -= delta.x * controller.mouse_sensitivity;
        controller.angle_y += delta.y * controller.mouse_sensitivity;

        // Clamp vertical angle to prevent flipping
        controller.angle_y = controller
            .angle_y
            .clamp(-89.0f32.to_radians(), 89.0f32.to_radians());

        let player_transform = player_query.single();
        let player_position = player_transform.translation;

        // Calculate new camera position
        let x = player_position.x
            + controller.distance * controller.angle_y.cos() * controller.angle_x.sin();
        let y = player_position.y + controller.distance * controller.angle_y.sin();
        let z = player_position.z
            + controller.distance * controller.angle_y.cos() * controller.angle_x.cos();

        let camera_position = Vec3::new(x, y, z);

        // Update camera transform
        *transform =
            Transform::from_translation(camera_position).looking_at(player_position, Vec3::Y);
    }
}
