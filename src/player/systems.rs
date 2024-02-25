use super::events::PlayerThrusterChanged;
use super::resources::{PlayerAssets, PlayerCollisionConvexShapes};
use super::{components::*, PlayerState, PLAYER_ACCELERATION, PLAYER_MAX_SPEED, PLAYER_SIZE};
use crate::asteroid::components::Asteroid;
use crate::health::Health;
use crate::{collision_groups::*, GameOver, NewGame};
use crate::{laser::events::SpawnLaser, score::resources::Score, star::components::Star};
use crate::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub fn load_player_assets(asset_server: Res<AssetServer>, mut player_assets: ResMut<PlayerAssets>) {
    player_assets.ship_texture = asset_server.load("images/sprites/playerShip1_red.png");
    player_assets.fire_texture = asset_server.load("images/sprites/fire13.png");
    player_assets.explosion_texture = asset_server.load("images/sprites/explosion.png");
    player_assets.explosion_sound = asset_server.load("audio/explosionCrunch_000.ogg");
    player_assets.star_sound = asset_server.load("audio/laserLarge_000.ogg");
}

pub fn new_game_spawn_player(
    mut new_game_reader: EventReader<NewGame>,
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    collision_shapes: Res<PlayerCollisionConvexShapes>,
    player_query: Query<Entity, With<Player>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    if new_game_reader.read().next().is_none() {
        return;
    }

    next_player_state.set(PlayerState::Alive);

    for entity in player_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let player_shapes = &collision_shapes.player_shapes;

    commands
        .spawn(PlayerBundle {
            player: Player {
                direction: Vec2::new(0.0, 1.0),
                velocity: Vec2::ZERO,
                take_damage: true,
            },
            health: Health { percent: 10 },
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(VIEWPORT_WIDTH / 2.0, VIEWPORT_HEIGHT / 2.0, 0.0),
                texture: player_assets.ship_texture.clone(),
                ..default()
            },
            player_collision_bundle: PlayerCollisionBundle {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::compound(
                    player_shapes
                        .iter()
                        .map(|vertices| {
                            (
                                Vec2::ZERO,
                                0.0,
                                Collider::convex_hull(vertices.as_ref()).unwrap(),
                            )
                        })
                        .collect(),
                ),
                collision_groups: CollisionGroups::new(
                    PLAYER_COLLISION_GROUP,
                    !LASER_COLLISION_GROUP,
                ),
                active_events: ActiveEvents::COLLISION_EVENTS | ActiveEvents::CONTACT_FORCE_EVENTS,
                colliding_entities: CollidingEntities::default(),
            },
        })
        .with_children(|parent| {
            parent.spawn(ForwardThruster).insert(SpriteBundle {
                transform: Transform::from_xyz(-20.0, -PLAYER_SIZE / 2.0 - 10.0, 0.0),
                texture: player_assets.fire_texture.clone(),
                visibility: Visibility::Hidden,
                ..default()
            });
            parent.spawn(ForwardThruster).insert(SpriteBundle {
                transform: Transform::from_xyz(20.0, -PLAYER_SIZE / 2.0 - 10.0, 0.0),
                texture: player_assets.fire_texture.clone(),
                visibility: Visibility::Hidden,
                ..default()
            });
            parent.spawn(BackwardThruster).insert(SpriteBundle {
                transform: Transform::from_xyz(-30.0, 20.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(PI)),
                texture: player_assets.fire_texture.clone(),
                visibility: Visibility::Hidden,
                ..default()
            });
            parent.spawn(BackwardThruster).insert(SpriteBundle {
                transform: Transform::from_xyz(30.0, 20.0, 0.0)
                    .with_rotation(Quat::from_rotation_z(PI)),
                texture: player_assets.fire_texture.clone(),
                visibility: Visibility::Hidden,
                ..default()
            });
        });
}

pub fn player_input(
    mut spawn_laser_writer: EventWriter<SpawnLaser>,
    mut thruster_writer: EventWriter<PlayerThrusterChanged>,
    mut player_query: Query<(&mut Player, &Transform), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut player, transform)) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            player.direction = player.direction.rotate(Vec2::from_angle(PI / 32.0));
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            player.direction = player.direction.rotate(Vec2::from_angle(-PI / 32.0));
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            let v = player.direction * PLAYER_ACCELERATION * time.delta_seconds();
            player.velocity += v;
            player.velocity = player.velocity.clamp_length_max(PLAYER_MAX_SPEED);

            thruster_writer.send(PlayerThrusterChanged::Forward);
        } else if keyboard_input.pressed(KeyCode::ArrowDown)
            || keyboard_input.pressed(KeyCode::KeyS)
        {
            let v = player.direction * PLAYER_ACCELERATION * time.delta_seconds();
            player.velocity -= v;
            player.velocity = player.velocity.clamp_length_max(PLAYER_MAX_SPEED);

            thruster_writer.send(PlayerThrusterChanged::Backward);
        } else {
            thruster_writer.send(PlayerThrusterChanged::None);
        }

        if keyboard_input.just_pressed(KeyCode::Space) {
            let Vec2 { x, y } = transform.translation.xy() + (player.direction * PLAYER_SIZE / 2.0);
            spawn_laser_writer.send(SpawnLaser {
                x,
                y,
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

pub fn wrap_player_movement(mut player_query: Query<&mut Transform, With<Player>>) {
    let half_player_size = PLAYER_SIZE / 2.0;
    let x_min = -half_player_size;
    let x_max = VIEWPORT_WIDTH + half_player_size;
    let y_min = -half_player_size;
    let y_max = VIEWPORT_HEIGHT + half_player_size;

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

pub fn player_hit_asteroid(
    mut commands: Commands,
    mut player_query: Query<(Entity, &mut Player, &CollidingEntities, &mut Health)>,
    asteroid_query: Query<Entity, With<Asteroid>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok((player_entity, mut player, colliding_entities, mut player_health)) =
        player_query.get_single_mut()
    {
        if !player.take_damage {
            return;
        }

        for colliding_entity in colliding_entities.iter() {
            if asteroid_query.contains(colliding_entity) {
                player_health.percent = player_health.percent.saturating_sub(10);

                if player_health.percent == 0 {
                    next_player_state.set(PlayerState::Dead);
                } else {
                    player.take_damage = false;
                    commands
                        .entity(player_entity)
                        .insert(DamageTimer(Timer::from_seconds(1.0, TimerMode::Once)));
                }
            }
        }
    }
}

pub fn player_death(
    player_assets: Res<PlayerAssets>,
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
) {
    for player_entity in player_query.iter() {
        commands.spawn(AudioBundle {
            source: player_assets.explosion_sound.clone(),
            settings: PlaybackSettings::ONCE,
        });

        commands.entity(player_entity).insert(Visibility::Hidden);

        commands
            .entity(player_entity)
            .insert(DeathTimer(Timer::from_seconds(1.5, TimerMode::Once)));
    }
}

pub fn player_death_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut DeathTimer), With<Player>>,
    mut game_over_writer: EventWriter<GameOver>,
) {
    if let Ok((player_entity, mut death_timer)) = player_query.get_single_mut() {
        death_timer.tick(time.delta());

        if death_timer.just_finished() {
            commands.entity(player_entity).despawn_recursive();
            game_over_writer.send(GameOver);
        }
    }
}

pub fn player_damage_timer(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(Entity, &mut Player, &mut DamageTimer), With<Player>>,
) {
    if let Ok((player_entity, mut player, mut damage_timer)) = player_query.get_single_mut() {
        damage_timer.tick(time.delta());

        if damage_timer.just_finished() {
            player.take_damage = true;
            commands.entity(player_entity).remove::<DamageTimer>();
        }
    }
}

pub fn player_hit_star(
    player_assets: Res<PlayerAssets>,
    mut commands: Commands,
    player_colliding_entities: Query<&CollidingEntities, With<Player>>,
    stars: Query<Entity, With<Star>>,
    mut score: ResMut<Score>,
) {
    if let Ok(colliding_entities) = player_colliding_entities.get_single() {
        for star_entity in stars.iter() {
            if colliding_entities.contains(star_entity) {
                score.value += 1;
                commands.spawn(AudioBundle {
                    source: player_assets.star_sound.clone(),
                    settings: PlaybackSettings::ONCE,
                });
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn forward_thruster_visibility(
    mut commands: Commands,
    mut thrust_changed_events: EventReader<PlayerThrusterChanged>,
    player_children_query: Query<&Children, With<Player>>,
    forward_thruster_query: Query<&ForwardThruster>,
    backward_thruster_query: Query<&BackwardThruster>,
) {
    if let Some(event) = thrust_changed_events.read().last() {
        if let Ok(children) = player_children_query.get_single() {
            match event {
                PlayerThrusterChanged::Forward => {
                    for &child in children.iter() {
                        if forward_thruster_query.contains(child) {
                            commands.entity(child).insert(Visibility::Inherited);
                        }
                    }
                }
                PlayerThrusterChanged::Backward => {
                    for &child in children.iter() {
                        if backward_thruster_query.contains(child) {
                            commands.entity(child).insert(Visibility::Inherited);
                        }
                    }
                }
                PlayerThrusterChanged::None => {
                    for &child in children.iter() {
                        if forward_thruster_query.contains(child) {
                            commands.entity(child).insert(Visibility::Hidden);
                        }
                        if backward_thruster_query.contains(child) {
                            commands.entity(child).insert(Visibility::Hidden);
                        }
                    }
                }
            }
        }
    }
}

pub fn spawn_player_explosion(
    mut commands: Commands,
    player_query: Query<&GlobalTransform, With<Player>>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Ok(player_global_transform) = player_query.get_single() {
        let texture = player_assets.explosion_texture.clone();
        let layout = TextureAtlasLayout::from_grid(Vec2::new(276.0, 306.5), 5, 2, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 9 };
        let player_location = player_global_transform.translation().xy();
        commands.spawn(PlayerExplosionBundle {
            player_explosion: PlayerExplosion,
            sprite_sheet_bundle: SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                visibility: Visibility::Visible,
                transform: Transform::from_xyz(player_location.x, player_location.y, 5.0),
                ..default()
            },
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(0.075, TimerMode::Repeating)),
        });
    }
}

pub fn animate_player_explosion(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<
        (
            Entity,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlas,
        ),
        With<PlayerExplosion>,
    >,
) {
    for (entity, indices, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if atlas.index == indices.last {
                commands.entity(entity).despawn_recursive();
            } else {
                atlas.index += 1
            };
        }
    }
}
