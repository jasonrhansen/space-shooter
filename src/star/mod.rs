pub mod components;
pub mod resources;
pub mod systems;

use crate::{AppState, UpdateSet, state::GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use resources::*;
use systems::*;

pub const NUM_STARS: usize = 10;
pub const STAR_SIZE: f32 = 15.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .configure_loading_state(
                LoadingStateConfig::new(AppState::Loading).load_collection::<StarAssets>(),
            )
            .add_systems(
                OnEnter(AppState::Running),
                new_game_spawn_stars.in_set(UpdateSet::Init),
            )
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Running).and(in_state(GameState::Playing))),
            );
    }
}
