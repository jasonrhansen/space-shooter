use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};

use super::components::*;
use super::events::SpawnLaser;
use super::LASER_SIZE;
use super::LASER_SPEED;

pub fn spawn_lasers(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnLaser>,
    asset_server: Res<AssetServer>,
) {
    event_reader.read().take(1).for_each(|spawn_laser| {
        let transform = Transform::from_xyz(spawn_laser.x, spawn_laser.y, 0.0).with_rotation(
            Quat::from_rotation_z(
                spawn_laser.direction.y.atan2(spawn_laser.direction.x) - PI / 2.0,
            ),
        );
        commands.spawn((
            SpriteBundle {
                transform,
                texture: asset_server.load("images/sprites/laserRed01.png"),
                ..default()
            },
            Laser {
                direction: spawn_laser.direction.normalize(),
            },
        ));
    });
}

pub fn laser_movement(mut laser_query: Query<(&mut Transform, &mut Laser)>, time: Res<Time>) {
    for (mut transform, laser) in laser_query.iter_mut() {
        let direction = Vec3::new(laser.direction.x, laser.direction.y, 0.0);
        transform.translation += direction * LASER_SPEED * time.delta_seconds();
    }
}

pub fn update_laser_direction(
    mut laser_query: Query<(&Transform, &mut Laser)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_laser_size = LASER_SIZE / 2.0;
    let x_min = half_laser_size;
    let x_max = window.width() - half_laser_size;
    let y_min = half_laser_size;
    let y_max = window.height() - half_laser_size;

    for (transform, mut laser) in laser_query.iter_mut() {
        let translation = transform.translation;
        if translation.x < x_min || translation.x > x_max {
            laser.direction.x *= -1.0;
        }

        let translation = transform.translation;
        if translation.y < y_min || translation.y > y_max {
            laser.direction.y *= -1.0;
        }
    }
}

pub fn despawn_offscreen_lasers(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_laser_size = LASER_SIZE / 2.0;
    let x_min = -half_laser_size;
    let x_max = window.width() + half_laser_size;
    let y_min = -half_laser_size;
    let y_max = window.height() + half_laser_size;

    for (entity, transform) in laser_query.iter() {
        if transform.translation.x < x_min
            || transform.translation.x > x_max
            || transform.translation.y < y_min
            || transform.translation.y > y_max
        {
            commands.entity(entity).despawn();
        }
    }
}
