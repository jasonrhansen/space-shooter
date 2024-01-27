use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

use crate::player::PLAYER_COLLISION_GROUP;

use super::components::*;
use super::resources::*;
use super::NUM_STARS;
use super::STAR_COLLISION_GROUP;
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

        spawn_star(Vec2::new(random_x, random_y), &mut commands, &asset_server);
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

        spawn_star(Vec2::new(random_x, random_y), &mut commands, &asset_server);
    }
}

fn spawn_star(position: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(Star {})
        .insert(SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, -9.0),
            texture: asset_server.load("images/sprites/star_gold.png"),
            ..default()
        })
        .insert(Sensor)
        .insert(Collider::ball(STAR_SIZE / 2.0))
        .insert(CollisionGroups::new(
            STAR_COLLISION_GROUP,
            PLAYER_COLLISION_GROUP,
        ));
}
