use std::f32::consts::PI;

use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::asteroid::components::Asteroid;

use super::components::*;
use super::events::SpawnLaser;
use super::LASER_SPEED;

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
            .insert(Collider::cuboid(4.0, 25.0))
            .insert(ActiveEvents::COLLISION_EVENTS);
    });
}

pub fn despawn_offscreen_lasers(
    mut commands: Commands,
    laser_query: Query<(Entity, &Transform), With<Laser>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    let max_offscreen = 20.0;
    let x_min = -max_offscreen;
    let x_max = window.width() + max_offscreen;
    let y_min = -max_offscreen;
    let y_max = window.height() + max_offscreen;

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
                        commands.entity(laser_entity).remove::<SpriteBundle>();
                    } else {
                        commands.entity(laser_entity).despawn();
                    }
                }
            }
        }
    }
}
