use super::LASER_SPEED;
use super::components::*;
use super::events::SpawnLaser;
use super::resources::LaserAssets;
use crate::asteroid::components::Asteroid;
use crate::physics_layer::*;
use crate::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::f32::consts::PI;

pub fn spawn_lasers(
    mut commands: Commands,
    mut spawn_laser_events: EventReader<SpawnLaser>,
    laser_assets: Res<LaserAssets>,
    audio: Res<Audio>,
) {
    spawn_laser_events.read().take(1).for_each(|spawn_laser| {
        audio.play(laser_assets.laser_sound.clone());

        let transform = Transform::from_xyz(spawn_laser.x, spawn_laser.y, -1.0).with_rotation(
            Quat::from_rotation_z(
                spawn_laser.direction.y.atan2(spawn_laser.direction.x) - PI / 2.0,
            ),
        );
        commands
            .spawn(Laser)
            .insert(Sprite::from_image(laser_assets.laser_texture.clone()))
            .insert(transform)
            .insert(LinearVelocity(
                spawn_laser.direction.normalize() * LASER_SPEED,
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::round_rectangle(4.0, 25.0, 0.5))
            .insert(CollisionLayers::new(GameLayer::Laser, [GameLayer::Default]))
            .insert(CollisionEventsEnabled);
    });
}

pub fn new_game_despawn_lasers(mut commands: Commands, lasers: Query<Entity, With<Laser>>) {
    for entity in lasers.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn despawn_offscreen_lasers(
    mut commands: Commands,
    lasers: Query<(Entity, &Transform), With<Laser>>,
) {
    let max_offscreen = 20.0;
    let x_min = -max_offscreen;
    let x_max = VIEWPORT_WIDTH + max_offscreen;
    let y_min = -max_offscreen;
    let y_max = VIEWPORT_HEIGHT + max_offscreen;

    for (entity, transform) in lasers.iter() {
        if transform.translation.x < x_min
            || transform.translation.x > x_max
            || transform.translation.y < y_min
            || transform.translation.y > y_max
        {
            commands.entity(entity).despawn();
        }
    }
}

pub fn laser_hit_asteroid(
    mut commands: Commands,
    mut collision_started_events: EventReader<CollisionStarted>,
    mut collision_ended_events: EventReader<CollisionEnded>,
    lasers: Query<Entity, With<Laser>>,
    asteroids: Query<Entity, With<Asteroid>>,
) {
    let laser_and_asteroid = |entity1: &Entity, entity2: &Entity| {
        if lasers.contains(*entity1) && asteroids.contains(*entity2) {
            Some((*entity1, *entity2))
        } else if lasers.contains(*entity2) && asteroids.contains(*entity1) {
            Some((*entity2, *entity1))
        } else {
            None
        }
    };

    for CollisionStarted(entity1, entity2) in collision_started_events.read() {
        if let Some((laser_entity, _)) = laser_and_asteroid(entity1, entity2) {
            // When colliding into an asteroid we don't want the laser to be visible.
            // but we still want it to continue the collision to exert a force.
            commands.entity(laser_entity).remove::<Sprite>();
        }
    }

    for CollisionEnded(entity1, entity2) in collision_ended_events.read() {
        if let Some((laser_entity, _)) = laser_and_asteroid(entity1, entity2) {
            // When done colliding we can despawn the laser.
            commands.entity(laser_entity).despawn();
        }
    }
}
