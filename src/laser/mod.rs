pub mod components;
pub mod events;
pub mod systems;

use self::events::SpawnLaser;
use crate::AppState;
use bevy::prelude::*;
use systems::*;

pub const LASER_SPEED: f32 = 800.0;
pub const LASER_SIZE: f32 = 64.0;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLaser>()
            .add_systems(
                Update,
                (despawn_offscreen_lasers, spawn_lasers).run_if(in_state(AppState::Playing)),
            )
            .add_systems(PostUpdate, laser_hit_asteroid);
    }
}
