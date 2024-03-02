use super::*;
use super::{components::*, resources::UiAssets};
use crate::{health::Health, player::components::Player, score::resources::Score};
use bevy::app::AppExit;
use bevy_kira_audio::prelude::*;

pub fn load_ui_assets(asset_server: Res<AssetServer>, mut ui_assets: ResMut<UiAssets>) {
    ui_assets.pause_sound = asset_server.load("audio/confirmation_001.ogg");
    ui_assets.resume_game_sound = asset_server.load("audio/confirmation_002.ogg");
}

pub fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Health: 100%",
            TextStyle {
                font_size: 32.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Left)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        }),
        ScoreText,
    ));
    commands.spawn((
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 32.0,
                ..default()
            },
        )
        .with_text_justify(JustifyText::Right)
        .with_style(Style {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            right: Val::Px(5.0),
            ..default()
        }),
        HealthText,
    ));
}

pub fn update_health_text(
    mut health_text_query: Query<&mut Text, With<HealthText>>,
    player_health_query: Query<&Health, With<Player>>,
) {
    if let Ok(health) = player_health_query.get_single() {
        if let Ok(mut text) = health_text_query.get_single_mut() {
            text.sections[0].value = format!("Health: {}%", health.percent);
        }
    }
}

pub fn update_score_text(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            text.sections[0].value = format!("Score: {}", score.value);
        }
    }
}

pub fn spawn_paused_screen(mut commands: Commands, ui_assets: Res<UiAssets>, audio: Res<Audio>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(15.0),
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            },
            PausedMenu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Paused",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                },
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(MENU_BUTTON_WIDTH),
                            height: Val::Px(MENU_BUTTON_HEIGHT),
                            ..default()
                        },
                        background_color: Color::rgba(0.0, 0.5, 0.0, 0.5).into(),
                        ..default()
                    },
                    ResumeGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Resume Game",
                        TextStyle {
                            font_size: 50.0,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(MENU_BUTTON_WIDTH),
                            height: Val::Px(MENU_BUTTON_HEIGHT),
                            ..default()
                        },
                        background_color: Color::rgba(0.5, 0.5, 0.0, 0.5).into(),
                        ..default()
                    },
                    NewGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "New Game",
                        TextStyle {
                            font_size: 50.0,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(MENU_BUTTON_WIDTH),
                            height: Val::Px(MENU_BUTTON_HEIGHT),
                            ..default()
                        },
                        background_color: Color::rgba(0.5, 0.0, 0.0, 0.5).into(),
                        ..default()
                    },
                    QuitGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font_size: 50.0,
                            ..default()
                        },
                    ));
                });
        });

    audio.play(ui_assets.pause_sound.clone());
}

pub fn despawn_paused_screen(
    mut commands: Commands,
    query: Query<Entity, With<PausedMenu>>,
    ui_assets: Res<UiAssets>,
    audio: Res<Audio>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
        audio.play(ui_assets.resume_game_sound.clone());
    }
}

pub fn button_interaction_color(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut background_color) in button_query.iter_mut() {
        let alpha = match interaction {
            Interaction::Pressed => 1.0,
            Interaction::Hovered => 0.75,
            Interaction::None => 0.5,
        };

        background_color.0.set_a(alpha);
    }
}

pub fn resume_game_button_action(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<ResumeGameButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    ui_assets: Res<UiAssets>,
    audio: Res<Audio>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Playing);
            audio.play(ui_assets.resume_game_sound.clone());
        }
    }
}

pub fn new_game_button_action(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            next_app_state.set(AppState::Loading);
            next_game_state.set(GameState::Playing);
        }
    }
}

pub fn quit_game_button_action(
    mut button_query: Query<&Interaction, (Changed<Interaction>, With<QuitGameButton>)>,
    mut exit: EventWriter<AppExit>,
) {
    if let Ok(interaction) = button_query.get_single_mut() {
        if *interaction == Interaction::Pressed {
            exit.send(AppExit);
        }
    }
}

pub fn spawn_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(15.0),
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.5).into(),
                ..default()
            },
            GameOverMenu,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Game Over!",
                TextStyle {
                    font_size: 100.0,
                    ..default()
                },
            ));
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(MENU_BUTTON_WIDTH),
                            height: Val::Px(MENU_BUTTON_HEIGHT),
                            ..default()
                        },
                        background_color: Color::rgba(0.0, 0.5, 0.0, 0.5).into(),
                        ..default()
                    },
                    NewGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "New Game",
                        TextStyle {
                            font_size: 50.0,
                            ..default()
                        },
                    ));
                });

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            width: Val::Px(MENU_BUTTON_WIDTH),
                            height: Val::Px(MENU_BUTTON_HEIGHT),
                            ..default()
                        },
                        background_color: Color::rgba(0.5, 0.0, 0.0, 0.5).into(),
                        ..default()
                    },
                    QuitGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Quit",
                        TextStyle {
                            font_size: 50.0,
                            ..default()
                        },
                    ));
                });
        });
}

pub fn despawn_game_over_screen(mut commands: Commands, query: Query<Entity, With<GameOverMenu>>) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
