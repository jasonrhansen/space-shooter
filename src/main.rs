#![allow(clippy::type_complexity)]

use crate::app_state::AppState;
use asteroid::AsteroidPlugin;
use bevy::window::PresentMode;
use bevy::{app::AppExit, prelude::*};
use bevy_rapier2d::prelude::*;
use bevy_rapier2d::{prelude::RapierConfiguration, render::DebugRenderContext};

pub mod app_state;
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
pub mod ui;

use background::spawn_background;
use camera::spawn_camera;
use laser::LaserPlugin;
use music::spawn_music;
use player::PlayerPlugin;
use score::resources::Score;
use score::ScorePlugin;
use star::StarPlugin;
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
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default().disabled(),
        ))
        .add_state::<AppState>()
        .add_event::<GameOver>()
        .add_systems(
            Startup,
            (setup_physics, spawn_camera, spawn_background, spawn_music),
        )
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
                exit_game,
                handle_game_over,
                update_paused_state,
                toggle_debug_render,
            ),
        )
        .add_systems(
            Update,
            handle_physics_active.run_if(state_changed::<AppState>()),
        )
        .run();
}

#[derive(Event)]
pub struct GameOver;

pub fn exit_game(mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_reader: EventReader<GameOver>, score: Res<Score>) {
    if game_over_reader.read().next().is_some() {
        println!("Game Over! Score: {}", score.value);
    }
}

pub fn update_paused_state(
    app_state: ResMut<State<AppState>>,
    mut next_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::Return) {
        if app_state.as_ref() == &AppState::Paused {
            next_app_state.set(AppState::Playing);
        } else if app_state.as_ref() == &AppState::Playing {
            next_app_state.set(AppState::Paused);
        }
    }
}

pub fn setup_physics(mut rapier_config: ResMut<RapierConfiguration>) {
    // Disable gravity
    rapier_config.gravity = Vec2::ZERO;
}

pub fn handle_physics_active(
    app_state: Res<State<AppState>>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    rapier_config.physics_pipeline_active = app_state.as_ref() == &AppState::Playing;
}

pub fn toggle_debug_render(
    mut debug_context: ResMut<DebugRenderContext>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F2) {
        debug_context.enabled = !debug_context.enabled;
    }
}
