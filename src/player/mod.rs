pub mod components;
pub mod events;
pub mod resources;
pub mod systems;

use self::{
    events::PlayerThrusterChanged,
    resources::{PlayerAssets, PlayerCollisionConvexShapes},
};
use crate::{state::GameState, AppState, UpdateSet};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
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
            .configure_loading_state(
                LoadingStateConfig::new(AppState::Loading).load_collection::<PlayerAssets>(),
            )
            .add_event::<PlayerThrusterChanged>()
            .add_systems(
                OnEnter(AppState::Running),
                new_game_spawn_player.in_set(UpdateSet::Init),
            )
            .add_systems(
                Update,
                (
                    (
                        player_input.in_set(UpdateSet::Input).after(UpdateSet::Init),
                        (player_movement, wrap_player_movement)
                            .chain()
                            .in_set(UpdateSet::Movement)
                            .after(UpdateSet::Input),
                        (player_hit_asteroid, player_hit_star)
                            .in_set(UpdateSet::Collision)
                            .after(UpdateSet::Movement),
                        thruster_visibility,
                        player_damage_timer,
                    )
                        .run_if(in_state(PlayerState::Alive)),
                    (player_death_timer, animate_player_explosion)
                        .run_if(in_state(PlayerState::Dead)),
                    thruster_sound,
                )
                    .run_if(in_state(AppState::Running).and_then(in_state(GameState::Playing))),
            )
            .add_systems(
                OnEnter(PlayerState::Dead),
                (player_death, spawn_player_explosion).after(UpdateSet::Ui),
            );
    }
}
