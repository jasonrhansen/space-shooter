use std::f32::consts::PI;

use super::{components::*, PLAYER_ACCELERATION, PLAYER_MAX_SPEED, PLAYER_SIZE};
use crate::{
    asteroid::{components::Asteroid, ASTEROID_SIZE},
    events::GameOver,
    laser::events::SpawnLaser,
    score::resources::Score,
    star::{components::Star, STAR_SIZE},
};
use bevy::{prelude::*, window::PrimaryWindow};

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("images/sprites/playerShip1_red.png"),
            ..default()
        },
        Player {
            direction: Vec2::new(0.0, 1.0),
            velocity: Vec2::ZERO,
        },
    ));
}

pub fn player_input(
    mut spawn_laser_writer: EventWriter<SpawnLaser>,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut player, transform)) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            player.direction = player.direction.rotate(Vec2::from_angle(PI / 32.0));
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            player.direction = player.direction.rotate(Vec2::from_angle(-PI / 32.0));
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            let v = player.direction * PLAYER_ACCELERATION * time.delta_seconds();
            player.velocity += v;
            player.velocity = player.velocity.clamp_length_max(PLAYER_MAX_SPEED);
        }

        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            let v = player.direction * PLAYER_ACCELERATION * time.delta_seconds();
            player.velocity -= v;
            player.velocity = player.velocity.clamp_length_max(PLAYER_MAX_SPEED);
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            spawn_laser_writer.send(SpawnLaser {
                x: transform.translation.x,
                y: transform.translation.y,
                direction: player.direction,
            });
        }
    }
}

pub fn player_movement(
    mut player_query: Query<(&Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform)) = player_query.get_single_mut() {
        transform.translation +=
            Vec3::new(player.velocity.x, player.velocity.y, 0.0) * time.delta_seconds();

        transform.rotation =
            Quat::from_rotation_z(player.direction.y.atan2(player.direction.x) - PI / 2.0);
    }
}

pub fn wrap_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let half_player_size = PLAYER_SIZE / 2.0;
    let x_min = -half_player_size;
    let x_max = window.width() + half_player_size;
    let y_min = -half_player_size;
    let y_max = window.height() + half_player_size;

    if let Ok(mut transform) = player_query.get_single_mut() {
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

#[derive(Component)]
struct SoundEffect;

pub fn player_hit_asteroid(
    mut commands: Commands,
    mut game_over_writer: EventWriter<GameOver>,
    player_query: Query<(Entity, &Transform), With<Player>>,
    asteroid_query: Query<&Transform, With<Asteroid>>,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        let player_radius = PLAYER_SIZE / 2.0;
        let asteroid_radius = ASTEROID_SIZE / 2.0;

        for asteroid_transform in asteroid_query.iter() {
            let distance = player_transform
                .translation
                .distance(asteroid_transform.translation);
            if distance < player_radius + asteroid_radius {
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                commands.spawn((
                    AudioBundle {
                        source: sound_effect,
                        settings: PlaybackSettings::ONCE,
                    },
                    SoundEffect,
                ));
                commands.entity(player_entity).despawn();
                game_over_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    star_query: Query<(Entity, &Transform), With<Star>>,
    asset_server: Res<AssetServer>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_radius = PLAYER_SIZE / 2.0;
        let star_radius = STAR_SIZE / 2.0;

        for (star_entity, star_transform) in star_query.iter() {
            let distance = player_transform
                .translation
                .distance(star_transform.translation);
            if distance < player_radius + star_radius {
                score.value += 1;
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                commands.spawn((
                    AudioBundle {
                        source: sound_effect,
                        settings: PlaybackSettings::ONCE,
                    },
                    SoundEffect,
                ));
                commands.entity(star_entity).despawn();
            }
        }
    }
}
