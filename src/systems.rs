use bevy::{
    app::AppExit,
    audio::{Volume, VolumeLevel},
    prelude::*,
    render::{
        camera::ScalingMode,
        texture::{ImageAddressMode, ImageLoaderSettings, ImageSampler, ImageSamplerDescriptor},
    },
    sprite::Anchor,
    window::PrimaryWindow,
};
use bevy_rapier2d::{prelude::RapierConfiguration, render::DebugRenderContext};

use crate::{app_state::AppState, events::GameOver, VIEWPORT_HEIGHT, VIEWPORT_WIDTH};

pub fn exit_game(mut exit: EventWriter<AppExit>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }
}

pub fn handle_game_over(mut game_over_reader: EventReader<GameOver>) {
    for game_over in game_over_reader.read() {
        println!("Game Over! Score: {}", game_over.score);
    }
}

#[derive(Component)]
struct GameCamera;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    let mut camera_bundle = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    };

    camera_bundle.projection.scaling_mode = ScalingMode::AutoMin {
        min_width: VIEWPORT_WIDTH,
        min_height: VIEWPORT_HEIGHT,
    };

    commands.spawn((GameCamera, camera_bundle));
}

#[derive(Component)]
struct Background;

pub fn spawn_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    let sampler_desc = ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        ..Default::default()
    };

    let settings = move |s: &mut ImageLoaderSettings| {
        s.sampler = ImageSampler::Descriptor(sampler_desc.clone());
    };

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, -1000.0),
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                rect: Some(Rect::new(0.0, 0.0, VIEWPORT_WIDTH, VIEWPORT_HEIGHT)),
                ..default()
            },
            texture: asset_server.load_with_settings("images/backgrounds/darkPurple.png", settings),
            ..default()
        },
        Background {},
    ));
}

#[derive(Component)]
struct Music;

pub fn spawn_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioBundle {
            source: asset_server.load("audio/sci-fi-dramatic-theme.ogg"),
            settings: PlaybackSettings::LOOP.with_volume(Volume::Relative(VolumeLevel::new(0.5))),
        },
        Music,
    ));
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
