use super::components::*;
use super::resources::AsteroidCollisionConvexShapes;
use super::ASTEROID_SIZE;
use super::ASTEROID_SPEED;
use super::NUM_ASTEROIDS;
use super::PLAYER_SAFE_RADIUS;
use crate::NewGame;
use crate::VIEWPORT_HEIGHT;
use crate::VIEWPORT_WIDTH;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use rand::prelude::*;
use std::f32::consts::PI;

pub fn new_game_spawn_asteroids(
    mut new_game_reader: EventReader<NewGame>,
    mut commands: Commands,
    asteroids_query: Query<Entity, With<Asteroid>>,
    asset_server: Res<AssetServer>,
    collision_shapes: Res<AsteroidCollisionConvexShapes>,
) {
    if new_game_reader.read().next().is_none() {
        return;
    }

    for entity in asteroids_query.iter() {
        commands.entity(entity).despawn();
    }

    let viewport_center = Vec2::new(VIEWPORT_WIDTH / 2.0, VIEWPORT_HEIGHT / 2.0);
    let asteroid_shapes = &collision_shapes.asteroid_shapes;

    for i in 0..NUM_ASTEROIDS {
        for color in ["Grey", "Brown"] {
            let position = {
                let random_position = Vec2::new(
                    random::<f32>() * VIEWPORT_WIDTH,
                    random::<f32>() * VIEWPORT_HEIGHT,
                );
                // Make sure the asteroid doesn't spawn too close to the player.
                if random_position.distance(viewport_center) < PLAYER_SAFE_RADIUS {
                    (random_position - viewport_center).normalize() * PLAYER_SAFE_RADIUS
                } else {
                    random_position
                }
            };

            commands
                .spawn(Asteroid)
                .insert(SpriteBundle {
                    transform: Transform::from_translation(position.extend(0.0)),
                    texture: asset_server.load(format!(
                        "images/sprites/meteor{}_big{}.png",
                        color,
                        i + 1
                    )),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Velocity {
                    linvel: Vec2::new(random::<f32>(), random::<f32>()).normalize()
                        * ASTEROID_SPEED,
                    angvel: random::<f32>() * PI - PI,
                })
                .insert(Collider::compound(
                    asteroid_shapes[i]
                        .iter()
                        .map(|vertices| {
                            (
                                Vec2::ZERO,
                                0.0,
                                Collider::convex_hull(vertices.as_ref()).unwrap(),
                            )
                        })
                        .collect(),
                ));
        }
    }
}

pub fn wrap_asteroid_movement(mut asteroid_query: Query<&mut Transform, With<Asteroid>>) {
    let half_asteroid_size = ASTEROID_SIZE / 2.0;
    let x_min = -half_asteroid_size;
    let x_max = VIEWPORT_WIDTH + half_asteroid_size;
    let y_min = -half_asteroid_size;
    let y_max = VIEWPORT_HEIGHT + half_asteroid_size;

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
