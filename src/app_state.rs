use bevy::prelude::States;

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Playing,
    Paused,
    GameOver,
}
