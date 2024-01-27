use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use bevy_rapier2d::geometry;
use resources::*;
use systems::*;

use crate::AppState;

pub const NUM_STARS: usize = 10;
pub const STAR_SIZE: f32 = 15.0;
pub const STAR_SPAWN_TIME: f32 = 1.0;
pub const STAR_COLLISION_GROUP: geometry::Group = geometry::Group::GROUP_3;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_systems(Startup, spawn_stars)
            .add_systems(
                Update,
                (tick_star_spawn_timer, spawn_stars_over_time).run_if(in_state(AppState::Playing)),
            );
    }
}
