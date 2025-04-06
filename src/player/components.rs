use bevy::prelude::*;
use bevy_kira_audio::AudioInstance;
use bevy_rapier2d::{
    dynamics::RigidBody,
    geometry::{ActiveEvents, Collider, CollidingEntities, CollisionGroups},
};

use crate::health::Health;

#[derive(Component, Clone, Debug, PartialEq)]
pub struct Player {
    pub direction: Vec2,
    pub velocity: Vec2,
    pub take_damage: bool,
}

#[derive(Component, Deref, DerefMut)]
pub struct DamageTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct DeathTimer(pub Timer);

#[derive(Component)]
pub struct ForwardThruster;

#[derive(Component)]
pub struct BackwardThruster;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub health: Health,
    pub sprite_bundle: SpriteBundle,
    pub player_collision_bundle: PlayerCollisionBundle,
    pub thruster_sound: ThrusterSound,
    pub name: Name,
}

#[derive(Bundle)]
pub struct PlayerCollisionBundle {
    pub rigid_body: RigidBody,
    pub collider: Collider,
    pub collision_groups: CollisionGroups,
    pub active_events: ActiveEvents,
    pub colliding_entities: CollidingEntities,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct PlayerExplosion;

#[derive(Bundle)]
pub struct PlayerExplosionBundle {
    pub player_explosion: PlayerExplosion,
    pub sprite_bundle: SpriteBundle,
    pub texture_atlas: TextureAtlas,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
}

#[derive(Component, Deref, DerefMut)]
pub struct ThrusterSound(pub Handle<AudioInstance>);
