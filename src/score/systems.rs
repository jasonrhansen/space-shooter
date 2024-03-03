use super::resources::*;
use crate::GameOver;
use bevy::prelude::*;

pub fn new_game_reset_score(mut score: ResMut<Score>) {
    score.value = 0;
}

pub fn update_high_scores(
    mut game_over_events: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
    score: Res<Score>,
) {
    if game_over_events.read().next().is_some() {
        high_scores.scores.push(("Player".to_string(), score.value));
        high_scores.scores.sort_by(|a, b| b.1.cmp(&a.1));
        high_scores.scores.truncate(5);
    }
}

pub fn high_scores_updated(high_scores: Res<HighScores>) {
    if high_scores.is_changed() {
        println!("High Scores:");
        for (i, score) in high_scores.scores.iter().enumerate() {
            println!("{}. {} - {}", i + 1, score.0, score.1);
        }
    }
}
