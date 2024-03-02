use bevy::prelude::States;

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    Running,
    GameOver,
}

#[derive(States, Debug, Default, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Playing,
    Paused,
}
