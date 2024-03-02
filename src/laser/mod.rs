pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use self::{events::SpawnLaser, resources::LaserAssets};
use crate::{state::GameState, AppState, UpdateSet};
use bevy::prelude::*;
use systems::*;

pub const LASER_SPEED: f32 = 800.0;
pub const LASER_SIZE: f32 = 64.0;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LaserAssets>()
            .add_event::<SpawnLaser>()
            .add_systems(Startup, load_laser_assets)
            .add_systems(
                OnEnter(AppState::Running),
                new_game_despawn_lasers.in_set(UpdateSet::Init),
            )
            .add_systems(
                Update,
                (despawn_offscreen_lasers, spawn_lasers)
                    .run_if(in_state(AppState::Running).and_then(in_state(GameState::Playing))),
            )
            .add_systems(
                PostUpdate,
                laser_hit_asteroid
                    .run_if(in_state(AppState::Running).and_then(in_state(GameState::Playing))),
            );
    }
}
