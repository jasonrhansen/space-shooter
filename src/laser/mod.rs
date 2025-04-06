pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use self::{events::SpawnLaser, resources::LaserAssets};
use crate::{AppState, UpdateSet, state::GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use systems::*;

pub const LASER_SPEED: f32 = 800.0;
pub const LASER_SIZE: f32 = 64.0;

pub struct LaserPlugin;

impl Plugin for LaserPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(AppState::Loading).load_collection::<LaserAssets>(),
        )
        .add_event::<SpawnLaser>()
        .add_systems(
            OnEnter(AppState::Running),
            new_game_despawn_lasers.in_set(UpdateSet::Init),
        )
        .add_systems(
            Update,
            (despawn_offscreen_lasers, spawn_lasers)
                .run_if(in_state(AppState::Running).and(in_state(GameState::Playing))),
        )
        .add_systems(
            PostUpdate,
            laser_hit_asteroid
                .run_if(in_state(AppState::Running).and(in_state(GameState::Playing))),
        );
    }
}
