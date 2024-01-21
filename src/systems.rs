use bevy::{app::AppExit, prelude::*, window::PrimaryWindow};

use crate::events::GameOver;

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

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}

#[derive(Component)]
struct Background;

pub fn spawn_background(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, -10.0),
            sprite: Sprite {
                custom_size: Some(Vec2::new(window.width(), window.height())),
                ..default()
            },
            texture: asset_server.load("images/backgrounds/darkPurple.png"),
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
            settings: PlaybackSettings::LOOP,
        },
        Music,
    ));
}
