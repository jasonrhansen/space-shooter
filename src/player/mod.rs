use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;

use bevy_rapier2d::{geometry, prelude::Vect};
use once_cell::sync::Lazy;
use systems::*;

use crate::AppState;

use self::events::PlayerThrusterChanged;

pub const PLAYER_SIZE: f32 = 75.0;
pub const PLAYER_MAX_SPEED: f32 = 800.0;
pub const PLAYER_ACCELERATION: f32 = 400.0;
pub const PLAYER_COLLISION_GROUP: geometry::Group = geometry::Group::GROUP_1;
pub static COLLISION_VERTICES: Lazy<[Box<[Vect]>; 5]> = Lazy::new(|| {
    [
        Box::new([
            Vec2::new(-8.0, 37.0),
            Vec2::new(8.0, 37.0),
            Vec2::new(15.0, -27.0),
            Vec2::new(9.0, -37.0),
            Vec2::new(-7.0, -37.0),
            Vec2::new(-13.0, -28.0),
        ]),
        Box::new([
            Vec2::new(13.0, 11.0),
            Vec2::new(36.0, -2.0),
            Vec2::new(36.0, -22.0),
            Vec2::new(15.0, -27.0),
        ]),
        Box::new([
            Vec2::new(-13.0, 11.0),
            Vec2::new(-36.0, -2.0),
            Vec2::new(-36.0, -22.0),
            Vec2::new(-15.0, -27.0),
        ]),
        Box::new([
            Vec2::new(36.0, -2.0),
            Vec2::new(49.0, 6.0),
            Vec2::new(36.0, -22.0),
            Vec2::new(46.0, -29.0),
        ]),
        Box::new([
            Vec2::new(-36.0, -2.0),
            Vec2::new(-49.0, 6.0),
            Vec2::new(-36.0, -22.0),
            Vec2::new(-46.0, -29.0),
        ]),
    ]
});

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlayerThrusterChanged>()
            .add_systems(Startup, spawn_player)
            .add_systems(
                Update,
                (
                    (player_input, player_movement, wrap_player_movement).chain(),
                    // player_hit_asteroid,
                    player_hit_star,
                    forward_thruster_visibility,
                )
                    .run_if(in_state(AppState::Playing)),
            );
    }
}
