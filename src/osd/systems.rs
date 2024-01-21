use bevy::prelude::*;

use crate::score::resources::Score;

use super::components::{PausedText, ScoreText};

pub fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 32.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        ScoreText,
    ));
}

pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            text.sections[0].value = format!("Score: {}", score.value);
        }
    }
}

pub fn show_paused_screen(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Paused",
            TextStyle {
                font_size: 200.0,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style { ..default() }),
        PausedText,
    ));
}

pub fn remove_paused_screen(mut commands: Commands, query: Query<Entity, With<PausedText>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
