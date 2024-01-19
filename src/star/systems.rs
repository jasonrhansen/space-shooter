use bevy::{prelude::*, window::PrimaryWindow};
use rand::prelude::*;

use crate::player::components::Player;
use crate::player::PLAYER_SIZE;
use crate::score::resources::Score;

use super::components::*;
use super::resources::*;
use super::NUM_STARS;
use super::STAR_SIZE;

pub fn spawn_stars(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUM_STARS {
        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("images/sprites/star_gold.png"),
                ..default()
            },
            Star {},
        ));
    }
}

#[derive(Component)]
struct StarSound;

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

                commands.spawn((
                    AudioBundle {
                        source: asset_server.load("audio/laserLarge_000.ogg"),
                        settings: PlaybackSettings::ONCE,
                    },
                    StarSound,
                ));
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let random_x = random::<f32>() * window.width();
        let random_y = random::<f32>() * window.height();
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0),
                texture: asset_server.load("images/sprites/star_gold.png"),
                ..default()
            },
            Star {},
        ));
    }
}
