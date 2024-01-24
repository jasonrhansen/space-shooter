use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use super::components::*;
use super::ASTEROID_SIZE;
use super::ASTEROID_SPEED;
use super::NUM_ASTEROIDS;

pub fn spawn_asteroids(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for i in 1..=NUM_ASTEROIDS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands
            .spawn(Asteroid)
            .insert(SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load(format!("images/sprites/meteorGrey_big{i}.png")),
                ..default()
            })
            .insert(RigidBody::Dynamic)
            .insert(Velocity {
                linvel: Vec2::new(random::<f32>(), random::<f32>()).normalize() * ASTEROID_SPEED,
                angvel: random::<f32>() * PI - PI,
            })
            .insert(Collider::ball(ASTEROID_SIZE / 2.0));
    }
}

pub fn wrap_asteroid_movement(
    mut asteroid_query: Query<&mut Transform, With<Asteroid>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_asteroid_size = ASTEROID_SIZE / 2.0;
    let x_min = -half_asteroid_size;
    let x_max = window.width() + half_asteroid_size;
    let y_min = -half_asteroid_size;
    let y_max = window.height() + half_asteroid_size;

    for mut transform in asteroid_query.iter_mut() {
        if transform.translation.x < x_min {
            transform.translation.x = x_max - 1.0;
        } else if transform.translation.x > x_max {
            transform.translation.x = x_min + 1.0;
        }

        if transform.translation.y < y_min {
            transform.translation.y = y_max - 1.0;
        } else if transform.translation.y > y_max {
            transform.translation.y = y_min + 1.0;
        }
    }
}
