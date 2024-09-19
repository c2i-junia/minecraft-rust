use bevy::prelude::*;
use bevy::window::WindowResolution;

// Taille de la fenêtre
const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 600.0;

// Vitesse des palettes
const PADDLE_SPEED: f32 = 500.0;
// Taille des palettes
const PADDLE_SIZE: Vec2 = Vec2::new(10.0, 100.0);
// Vitesse initiale de la balle
const BALL_SPEED: f32 = 400.0;
// Taille de la balle
const BALL_SIZE: Vec2 = Vec2::new(10.0, 10.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Pong Game".to_string(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..Default::default()
            }),
            ..Default::default()
        }))
        // Mise à jour des systèmes avec la nouvelle syntaxe
        .add_systems(Startup, (setup_camera, setup_system))
        .add_systems(Update, (
            paddle_movement_system,
            ball_movement_system,
            ball_collision_system,
        ))
        .run();
}

// Composant pour les palettes
#[derive(Component)]
struct Paddle {
    side: Side,
}

// Côté de la palette (gauche ou droite)
#[derive(PartialEq)]
enum Side {
    Left,
    Right,
}

// Composant pour la balle
#[derive(Component)]
struct Ball {
    velocity: Vec3,
}

// Configuration initiale du jeu
fn setup_system(mut commands: Commands) {
    // Palette gauche
    commands.spawn((
        Paddle { side: Side::Left },
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(PADDLE_SIZE),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(-WINDOW_WIDTH / 2.0 + PADDLE_SIZE.x, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    // Palette droite
    commands.spawn((
        Paddle { side: Side::Right },
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(PADDLE_SIZE),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(WINDOW_WIDTH / 2.0 - PADDLE_SIZE.x, 0.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        },
    ));

    // Balle
    commands.spawn((
        Ball {
            velocity: Vec3::new(-BALL_SPEED, -BALL_SPEED / 2.0, 0.0),
        },
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(BALL_SIZE),
                ..Default::default()
            },
            transform: Transform::default(),
            ..Default::default()
        },
    ));
}

// Configuration de la caméra
fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

// Système de mouvement des palettes
fn paddle_movement_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Paddle, &mut Transform)>,
    time: Res<Time>,
) {
    for (paddle, mut transform) in query.iter_mut() {
        let mut direction = 0.0;
        if paddle.side == Side::Left {
            if keyboard_input.pressed(KeyCode::W) {
                direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::S) {
                direction -= 1.0;
            }
        } else {
            if keyboard_input.pressed(KeyCode::Up) {
                direction += 1.0;
            }
            if keyboard_input.pressed(KeyCode::Down) {
                direction -= 1.0;
            }
        }

        transform.translation.y += direction * PADDLE_SPEED * time.delta_seconds();

        // Limiter les palettes à l'écran
        let half_paddle_height = PADDLE_SIZE.y / 2.0;
        let limit = WINDOW_HEIGHT / 2.0 - half_paddle_height;
        transform.translation.y = transform.translation.y.clamp(-limit, limit);
    }
}

// Système de mouvement de la balle
fn ball_movement_system(
    mut ball_query: Query<(&mut Transform, &Ball)>,
    time: Res<Time>,
) {
    for (mut transform, ball) in ball_query.iter_mut() {
        transform.translation += ball.velocity * time.delta_seconds();
    }
}

// Système de collision de la balle
fn ball_collision_system(
    mut ball_query: Query<(&mut Ball, &mut Transform, &Sprite)>,
    paddle_query: Query<(&Paddle, &Transform, &Sprite), Without<Ball>>, // Ajout de Without<Ball>
) {
    let (mut ball, mut ball_transform, ball_sprite) = ball_query.single_mut();
    let ball_size = ball_sprite.custom_size.unwrap();

    // Collision avec le haut et le bas de l'écran
    let half_ball_size = ball_size.y / 2.0;
    let top_limit = WINDOW_HEIGHT / 2.0 - half_ball_size;
    let bottom_limit = -WINDOW_HEIGHT / 2.0 + half_ball_size;

    if ball_transform.translation.y >= top_limit || ball_transform.translation.y <= bottom_limit {
        ball.velocity.y = -ball.velocity.y;
    }

    // Collision avec les palettes
    for (_paddle, paddle_transform, paddle_sprite) in paddle_query.iter() {
        let paddle_size = paddle_sprite.custom_size.unwrap();
        let collision = collide(
            ball_transform.translation,
            ball_size,
            paddle_transform.translation,
            paddle_size,
        );

        if collision.is_some() {
            ball.velocity.x = -ball.velocity.x;
        }
    }

    // Réinitialisation de la balle si elle sort de l'écran
    let half_ball_width = ball_size.x / 2.0;
    let left_limit = -WINDOW_WIDTH / 2.0 - half_ball_width;
    let right_limit = WINDOW_WIDTH / 2.0 + half_ball_width;

    if ball_transform.translation.x < left_limit || ball_transform.translation.x > right_limit {
        // Réinitialiser la position de la balle
        ball_transform.translation = Vec3::ZERO;
        // Inverser la direction de la balle
        ball.velocity.x = -ball.velocity.x;
    }
}

// Fonction utilitaire pour détecter les collisions AABB
fn collide(
    pos_a: Vec3,
    size_a: Vec2,
    pos_b: Vec3,
    size_b: Vec2,
) -> Option<Vec3> {
    let collision_x = (pos_a.x - pos_b.x).abs() < (size_a.x / 2.0 + size_b.x / 2.0);
    let collision_y = (pos_a.y - pos_b.y).abs() < (size_a.y / 2.0 + size_b.y / 2.0);

    if collision_x && collision_y {
        Some(Vec3::new(pos_b.x, pos_b.y, 0.0))
    } else {
        None
    }
}

