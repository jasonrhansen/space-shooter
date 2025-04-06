use super::components::*;
use super::events::SpawnLaser;
use super::resources::LaserAssets;
use super::LASER_SPEED;
use crate::asteroid::components::Asteroid;
use crate::collision_groups::*;
use crate::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_rapier2d::prelude::*;
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
            .insert(Velocity::linear(
                spawn_laser.direction.normalize() * LASER_SPEED,
            ))
            .insert(RigidBody::Dynamic)
            .insert(Collider::round_cuboid(4.0, 25.0, 0.5))
            .insert(CollisionGroups::new(
                LASER_COLLISION_GROUP,
                !(PLAYER_COLLISION_GROUP | LASER_COLLISION_GROUP),
            ))
            .insert(ActiveEvents::COLLISION_EVENTS);
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
    mut collision_events: EventReader<CollisionEvent>,
    lasers: Query<Entity, With<Laser>>,
    asteroids: Query<Entity, With<Asteroid>>,
) {
    for collision_event in collision_events.read() {
        let (is_started, entity1, entity2) = match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => (true, *entity1, *entity2),
            CollisionEvent::Stopped(entity1, entity2, _flags) => (false, *entity1, *entity2),
        };
        for laser_entity in lasers.iter() {
            for asteroid_entity in asteroids.iter() {
                if entity1 == laser_entity && entity2 == asteroid_entity
                    || entity1 == asteroid_entity && entity2 == laser_entity
                {
                    if is_started {
                        // When colliding into an asteroid we don't want the laser to be visible.
                        // but we still want it to continue the collision to exert a force.
                        commands.entity(laser_entity).remove::<Sprite>();
                    } else {
                        // When done colliding we can despawn the laser.
                        commands.entity(laser_entity).despawn();
                    }
                }
            }
        }
    }
}
