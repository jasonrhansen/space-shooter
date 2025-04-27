use super::events::PlayerThrusterChanged;
use super::resources::{PlayerAssets, PlayerCollisionConvexShapes};
use super::{PLAYER_ACCELERATION, PLAYER_MAX_SPEED, PLAYER_SIZE, PlayerState, components::*};
use crate::GameOver;
use crate::asteroid::components::Asteroid;
use crate::health::Health;
use crate::physics_layer::GameLayer;
use crate::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use crate::{laser::events::SpawnLaser, score::resources::Score, star::components::Star};
use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use std::f32::consts::PI;

pub fn new_game_spawn_player(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
    player_assets: Res<PlayerAssets>,
    collision_shapes: Res<PlayerCollisionConvexShapes>,
    audio: Res<Audio>,
) {
    next_player_state.set(PlayerState::Alive);

    for entity in player.iter() {
        commands.entity(entity).despawn_recursive();
    }

    let player_shapes = &collision_shapes.player_shapes;

    commands
        .spawn(PlayerBundle {
            name: Name::new("Player"),
            player: Player {
                direction: Vec2::new(0.0, 1.0),
                velocity: Vec2::ZERO,
                take_damage: true,
            },
            health: Health { percent: 10 },
            sprite: Sprite::from_image(player_assets.ship_texture.clone()),
            transform: Transform::from_xyz(VIEWPORT_WIDTH / 2.0, VIEWPORT_HEIGHT / 2.0, 0.0),
            player_collision_bundle: PlayerCollisionBundle {
                rigid_body: RigidBody::Dynamic,
                collider: Collider::compound(
                    player_shapes
                        .iter()
                        .map(|vertices| {
                            (
                                Vec2::ZERO,
                                0.0,
                                Collider::convex_hull(vertices.clone()).unwrap(),
                            )
                        })
                        .collect(),
                ),
                collision_layers: CollisionLayers::new(
                    GameLayer::Player,
                    [GameLayer::Default, GameLayer::Star],
                ),
                colliding_entities: CollidingEntities::default(),
            },
            thruster_sound: ThrusterSound(
                audio
                    .play(player_assets.thruster_sound.clone())
                    .paused()
                    .looped()
                    .handle(),
            ),
        })
        .with_children(|parent| {
            parent
                .spawn(ForwardThruster)
                .insert(Sprite::from_image(player_assets.fire_texture.clone()))
                .insert(Transform::from_xyz(-20.0, -PLAYER_SIZE / 2.0 - 10.0, 0.0))
                .insert(Visibility::Hidden);
            parent
                .spawn(ForwardThruster)
                .insert(Sprite::from_image(player_assets.fire_texture.clone()))
                .insert(Transform::from_xyz(20.0, -PLAYER_SIZE / 2.0 - 10.0, 0.0))
                .insert(Visibility::Hidden);
            parent
                .spawn(BackwardThruster)
                .insert(Sprite::from_image(player_assets.fire_texture.clone()))
                .insert(Transform::from_xyz(-30.0, 20.0, 0.0))
                .insert(Visibility::Hidden);
            parent
                .spawn(BackwardThruster)
                .insert(Sprite::from_image(player_assets.fire_texture.clone()))
                .insert(Transform::from_xyz(30.0, 20.0, 0.0))
                .insert(Visibility::Hidden);
        });
}

pub fn player_input(
    mut spawn_laser_writer: EventWriter<SpawnLaser>,
    mut thruster_writer: EventWriter<PlayerThrusterChanged>,
    mut player: Query<(&mut Player, &Transform), With<Player>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Ok((mut player, transform)) = player.get_single_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA) {
            player.direction = player.direction.rotate(Vec2::from_angle(PI / 32.0));
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD) {
            player.direction = player.direction.rotate(Vec2::from_angle(-PI / 32.0));
        }

        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
            let v = player.direction * PLAYER_ACCELERATION * time.delta_secs();
            player.velocity += v;
            player.velocity = player.velocity.clamp_length_max(PLAYER_MAX_SPEED);

            thruster_writer.send(PlayerThrusterChanged::Forward);
        } else if keyboard_input.pressed(KeyCode::ArrowDown)
            || keyboard_input.pressed(KeyCode::KeyS)
        {
            let v = player.direction * PLAYER_ACCELERATION * time.delta_secs();
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
    mut player: Query<(&Player, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((player, mut transform)) = player.get_single_mut() {
        transform.translation +=
            Vec3::new(player.velocity.x, player.velocity.y, 0.0) * time.delta_secs();

        transform.rotation =
            Quat::from_rotation_z(player.direction.y.atan2(player.direction.x) - PI / 2.0);
    }
}

pub fn wrap_player_movement(mut player: Query<&mut Transform, With<Player>>) {
    let half_player_size = PLAYER_SIZE / 2.0;
    let x_min = -half_player_size;
    let x_max = VIEWPORT_WIDTH + half_player_size;
    let y_min = -half_player_size;
    let y_max = VIEWPORT_HEIGHT + half_player_size;

    if let Ok(mut transform) = player.get_single_mut() {
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
    mut player: Query<(Entity, &mut Player, &CollidingEntities, &mut Health)>,
    asteroids: Query<Entity, With<Asteroid>>,
    mut next_player_state: ResMut<NextState<PlayerState>>,
) {
    if let Ok((player_entity, mut player, colliding_entities, mut player_health)) =
        player.get_single_mut()
    {
        if !player.take_damage {
            return;
        }

        for colliding_entity in colliding_entities.iter() {
            if asteroids.contains(*colliding_entity) {
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
    mut commands: Commands,
    mut thruster_writer: EventWriter<PlayerThrusterChanged>,
    player: Query<Entity, With<Player>>,
    audio: Res<Audio>,
    player_assets: Res<PlayerAssets>,
) {
    for player_entity in player.iter() {
        thruster_writer.send(PlayerThrusterChanged::None);

        audio.play(player_assets.explosion_sound.clone());

        commands.entity(player_entity).insert(Visibility::Hidden);

        commands
            .entity(player_entity)
            .insert(DeathTimer(Timer::from_seconds(1.5, TimerMode::Once)));
    }
}

pub fn player_death_timer(
    mut commands: Commands,
    mut game_over_writer: EventWriter<GameOver>,
    mut player: Query<(Entity, &mut DeathTimer), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((player_entity, mut death_timer)) = player.get_single_mut() {
        death_timer.tick(time.delta());

        if death_timer.just_finished() {
            commands.entity(player_entity).despawn_recursive();
            game_over_writer.send(GameOver);
        }
    }
}

pub fn player_damage_timer(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Player, &mut DamageTimer), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((player_entity, mut player, mut damage_timer)) = player.get_single_mut() {
        damage_timer.tick(time.delta());

        if damage_timer.just_finished() {
            player.take_damage = true;
            commands.entity(player_entity).remove::<DamageTimer>();
        }
    }
}

pub fn player_hit_star(
    mut commands: Commands,
    player_colliding_entities: Query<&CollidingEntities, With<Player>>,
    stars: Query<Entity, With<Star>>,
    mut score: ResMut<Score>,
    audio: Res<Audio>,
    player_assets: Res<PlayerAssets>,
) {
    if let Ok(colliding_entities) = player_colliding_entities.get_single() {
        for star_entity in stars.iter() {
            if colliding_entities.contains(&star_entity) {
                score.value += 1;
                audio.play(player_assets.star_sound.clone());
                commands.entity(star_entity).despawn();
            }
        }
    }
}

pub fn thruster_visibility(
    mut commands: Commands,
    mut thrust_changed_events: EventReader<PlayerThrusterChanged>,
    player_children: Query<&Children, With<Player>>,
    forward_thruster: Query<&ForwardThruster>,
    backward_thruster: Query<&BackwardThruster>,
) {
    if let Some(event) = thrust_changed_events.read().last() {
        if let Ok(children) = player_children.get_single() {
            match event {
                PlayerThrusterChanged::Forward => {
                    for &child in children.iter() {
                        if forward_thruster.contains(child) {
                            commands.entity(child).insert(Visibility::Inherited);
                        }
                    }
                }
                PlayerThrusterChanged::Backward => {
                    for &child in children.iter() {
                        if backward_thruster.contains(child) {
                            commands.entity(child).insert(Visibility::Inherited);
                        }
                    }
                }
                PlayerThrusterChanged::None => {
                    for &child in children.iter() {
                        if forward_thruster.contains(child) {
                            commands.entity(child).insert(Visibility::Hidden);
                        }
                        if backward_thruster.contains(child) {
                            commands.entity(child).insert(Visibility::Hidden);
                        }
                    }
                }
            }
        }
    }
}

pub fn thruster_sound(
    mut thrust_changed_events: EventReader<PlayerThrusterChanged>,
    thruster_sound: Query<&ThrusterSound, With<Player>>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Ok(thruster_handle) = thruster_sound.get_single() {
        if let Some(instance) = audio_instances.get_mut(&thruster_handle.0) {
            if let Some(event) = thrust_changed_events.read().last() {
                match event {
                    PlayerThrusterChanged::Forward | PlayerThrusterChanged::Backward => {
                        instance.resume(AudioTween::default());
                    }
                    PlayerThrusterChanged::None => {
                        instance.stop(AudioTween::default());
                    }
                }
            }
        }
    }
}

pub fn spawn_player_explosion(
    mut commands: Commands,
    player: Query<&GlobalTransform, With<Player>>,
    player_assets: Res<PlayerAssets>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Ok(player_global_transform) = player.get_single() {
        let image = player_assets.explosion_texture.clone();
        let layout = TextureAtlasLayout::from_grid(UVec2::new(276, 306), 5, 2, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 9 };
        let player_location = player_global_transform.translation().xy();
        commands.spawn(PlayerExplosionBundle {
            player_explosion: PlayerExplosion,
            sprite: Sprite {
                image,
                texture_atlas: Some(TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                }),
                ..default()
            },
            transform: Transform::from_xyz(player_location.x, player_location.y, 5.0),
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(0.075, TimerMode::Repeating)),
        });
    }
}

pub fn animate_player_explosion(
    mut commands: Commands,
    mut explosion: Query<
        (Entity, &AnimationIndices, &mut AnimationTimer, &mut Sprite),
        With<PlayerExplosion>,
    >,
    time: Res<Time>,
) {
    for (entity, indices, mut timer, mut sprite) in &mut explosion {
        if let Some(atlas) = sprite.texture_atlas.as_mut() {
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
}
