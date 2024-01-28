use super::events::SpawnLaser;
use super::LASER_SPEED;
use super::{components::*, LASER_COLLISION_GROUP};
use crate::asteroid::components::Asteroid;
use crate::player::PLAYER_COLLISION_GROUP;
use crate::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub fn spawn_lasers(
    mut commands: Commands,
    mut event_reader: EventReader<SpawnLaser>,
    asset_server: Res<AssetServer>,
) {
    event_reader.read().take(1).for_each(|spawn_laser| {
        let sound_effect = asset_server.load("audio/sfx_laser1.ogg");
        commands.spawn(AudioBundle {
            source: sound_effect,
            settings: PlaybackSettings::ONCE,
        });

        let transform = Transform::from_xyz(spawn_laser.x, spawn_laser.y, -1.0).with_rotation(
            Quat::from_rotation_z(
                spawn_laser.direction.y.atan2(spawn_laser.direction.x) - PI / 2.0,
            ),
        );
        commands
            .spawn(Laser)
            .insert(SpriteBundle {
                transform,
                texture: asset_server.load("images/sprites/laserRed01.png"),
                ..default()
            })
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

pub fn despawn_offscreen_lasers(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
) {
    let max_offscreen = 20.0;
    let x_min = -max_offscreen;
    let x_max = VIEWPORT_WIDTH + max_offscreen;
    let y_min = -max_offscreen;
    let y_max = VIEWPORT_HEIGHT + max_offscreen;

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

pub fn laser_hit_asteroid(
    mut commands: Commands,
    mut event_reader: EventReader<CollisionEvent>,
    laser_query: Query<Entity, With<Laser>>,
    asteroid_query: Query<Entity, With<Asteroid>>,
) {
    for collision_event in event_reader.read() {
        let (is_started, entity1, entity2) = match collision_event {
            CollisionEvent::Started(entity1, entity2, _flags) => (true, *entity1, *entity2),
            CollisionEvent::Stopped(entity1, entity2, _flags) => (false, *entity1, *entity2),
        };
        for laser_entity in laser_query.iter() {
            for asteroid_entity in asteroid_query.iter() {
                if entity1 == laser_entity && entity2 == asteroid_entity
                    || entity1 == asteroid_entity && entity2 == laser_entity
                {
                    if is_started {
                        // When colliding into an asteroid we don't want the laser to be visible.
                        // but we still want it to continue the collision to exert a force.
                        commands.entity(laser_entity).remove::<SpriteBundle>();
                    } else {
                        // When done colliding we can despawn the laser.
                        commands.entity(laser_entity).despawn();
                    }
                }
            }
        }
    }
}
