use super::resources::*;
use crate::GameOver;
use bevy::prelude::*;

pub fn update_high_scores(
    mut game_over_reader: EventReader<GameOver>,
    mut high_scores: ResMut<HighScores>,
) {
    for game_over in game_over_reader.read() {
        high_scores
            .scores
            .push(("Player".to_string(), game_over.score));
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
