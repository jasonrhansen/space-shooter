pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use self::{
    events::PlayerThrusterChanged,
    resources::{PlayerAssets, PlayerCollisionConvexShapes},
};
use crate::{AppState, UpdateSet};
use bevy::prelude::*;
use systems::*;

pub const PLAYER_SIZE: f32 = 75.0;
pub const PLAYER_MAX_SPEED: f32 = 800.0;
pub const PLAYER_ACCELERATION: f32 = 400.0;

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum PlayerState {
    #[default]
    Alive,
    Dead,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PlayerState>()
            .init_resource::<PlayerCollisionConvexShapes>()
            .init_resource::<PlayerAssets>()
            .add_event::<PlayerThrusterChanged>()
            .add_systems(Startup, load_player_assets)
            .add_systems(
                Update,
                (
                    new_game_spawn_player.in_set(UpdateSet::Init),
                    (
                        player_input.in_set(UpdateSet::Input).after(UpdateSet::Init),
                        (player_movement, wrap_player_movement)
                            .chain()
                            .in_set(UpdateSet::Movement)
                            .after(UpdateSet::Input),
                        (player_hit_asteroid, player_hit_star)
                            .in_set(UpdateSet::Collision)
                            .after(UpdateSet::Movement),
                        forward_thruster_visibility,
                        player_damage_timer,
                    )
                        .run_if(in_state(PlayerState::Alive)),
                    (player_death_timer, animate_player_explosion)
                        .run_if(in_state(PlayerState::Dead)),
                )
                    .run_if(in_state(AppState::Playing)),
            )
            .add_systems(
                OnEnter(PlayerState::Dead),
                (player_death, spawn_player_explosion).after(UpdateSet::Ui),
            );
    }
}
