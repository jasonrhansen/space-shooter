#![allow(clippy::type_complexity)]

use crate::state::AppState;
use asteroid::AsteroidPlugin;
use bevy::app::AppExit;
use bevy::input::common_conditions::input_toggle_active;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_asset_loader::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::{prelude::RapierConfiguration, render::DebugRenderContext};

pub mod asteroid;
pub mod background;
pub mod camera;
pub mod collision_groups;
pub mod health;
pub mod laser;
pub mod music;
pub mod player;
pub mod score;
pub mod star;
pub mod state;
pub mod ui;

use background::{spawn_background, BackgroundAssets};
use camera::spawn_camera;
use health::Health;
use laser::LaserPlugin;
use music::{spawn_music, MusicAssets};
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use state::GameState;
use ui::UiPlugin;

pub const VIEWPORT_WIDTH: f32 = 1280.0;
pub const VIEWPORT_HEIGHT: f32 = 720.0;

// A ConvexShape is a list of vertices that define a convex shape.
pub type ConvexShape = Box<[Vect]>;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space Shooter".into(),
                    present_mode: PresentMode::AutoVsync,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
            AudioPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default().disabled(),
            WorldInspectorPlugin::default().run_if(input_toggle_active(false, KeyCode::F2)),
        ))
        .init_state::<AppState>()
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(AppState::Loading)
                .continue_to_state(AppState::Running)
                .load_collection::<MusicAssets>()
                .load_collection::<BackgroundAssets>(),
        )
        .register_type::<Health>()
        .add_event::<GameOver>()
        .add_systems(Startup, (setup_physics, spawn_camera))
        .add_systems(OnEnter(AppState::Running), (spawn_background, spawn_music))
        .add_plugins((
            AsteroidPlugin,
            PlayerPlugin,
            ScorePlugin,
            StarPlugin,
            LaserPlugin,
            UiPlugin,
        ))
        .add_systems(
            Update,
            (
                handle_physics_active
                    .run_if(state_changed::<AppState>.or_else(state_changed::<GameState>)),
                exit_game,
                update_paused_state.run_if(in_state(AppState::Running)),
                toggle_debug_render,
            ),
        )
        .add_systems(PostUpdate, handle_game_over)
        .run();
}

#[derive(Event)]
pub struct GameOver;

pub fn exit_game(mut exit_writer: EventWriter<AppExit>, keyboard_input: Res<ButtonInput<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit_writer.send(AppExit::Success);
    }
}

pub fn handle_game_over(
    mut game_over_events: EventReader<GameOver>,
    mut next_app_state: ResMut<NextState<AppState>>,
) {
    if game_over_events.read().next().is_some() {
        next_app_state.set(AppState::GameOver);
    }
}

pub fn update_paused_state(
    mut next_game_state: ResMut<NextState<GameState>>,
    game_state: Res<State<GameState>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Enter) {
        if game_state.as_ref() == &GameState::Paused {
            next_game_state.set(GameState::Playing);
        } else if game_state.as_ref() == &GameState::Playing {
            next_game_state.set(GameState::Paused);
        }
    }
}

pub fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    // Disable gravity
    rapier_config.gravity = Vec2::ZERO;
}

pub fn handle_physics_active(
    mut rapier_config: ResMut<RapierConfiguration>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
) {
    rapier_config.physics_pipeline_active =
        app_state.as_ref() == &AppState::Running && game_state.as_ref() == &GameState::Playing;
}

pub fn toggle_debug_render(
    mut debug_context: ResMut<DebugRenderContext>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F3) {
        debug_context.enabled = !debug_context.enabled;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum UpdateSet {
    Init,
    /// Input handling.
    Input,
    /// Everything that moves things (works with transforms).
    Movement,
    /// Collision detection and resolution.
    Collision,
    /// Systems that update the user interface.
    Ui,
}
