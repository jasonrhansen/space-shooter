use super::components::*;
use super::resources::*;
use super::*;
use crate::collision_groups::*;
use crate::VIEWPORT_HEIGHT;
use crate::VIEWPORT_WIDTH;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;

pub fn spawn_stars(mut commands: Commands, asset_server: Res<AssetServer>) {
    for _ in 0..NUM_STARS {
        let random_x = random::<f32>() * VIEWPORT_WIDTH;
        let random_y = random::<f32>() * VIEWPORT_HEIGHT;

        spawn_star(Vec2::new(random_x, random_y), &mut commands, &asset_server);
    }
}

pub fn tick_star_spawn_timer(mut star_spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    star_spawn_timer.timer.tick(time.delta());
}

pub fn spawn_stars_over_time(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let random_x = random::<f32>() * VIEWPORT_HEIGHT;
        let random_y = random::<f32>() * VIEWPORT_HEIGHT;

        spawn_star(Vec2::new(random_x, random_y), &mut commands, &asset_server);
    }
}

fn spawn_star(position: Vec2, commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands
        .spawn(Star {})
        .insert(SpriteBundle {
            transform: Transform::from_xyz(position.x, position.y, -9.0)
                .with_scale(Vec3::splat(0.5)),
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
