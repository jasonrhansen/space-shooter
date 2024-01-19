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

pub fn spawn_camera(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.get_single_mut().unwrap();

    window.resolution.set(800.0, 600.0);

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        ..default()
    });
}
