pub mod components;
pub mod resources;
pub mod systems;

use crate::{state::GameState, AppState, UpdateSet};
use bevy::prelude::*;
use resources::*;
use systems::*;

pub const NUM_STARS: usize = 10;
pub const STAR_SIZE: f32 = 15.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .init_resource::<StarAssets>()
            .add_systems(Startup, load_star_assets)
            .add_systems(
                OnEnter(AppState::Running),
                new_game_spawn_stars.in_set(UpdateSet::Init),
            )
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time)
                    .run_if(in_state(AppState::Running).and_then(in_state(GameState::Playing))),
            );
    }
}
