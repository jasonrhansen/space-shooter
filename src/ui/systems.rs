use super::components::*;
use super::*;
use crate::{score::resources::Score, AppState};

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

pub fn spawn_paused_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
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

    let sound_effect = asset_server.load("audio/confirmation_001.ogg");
    commands.spawn(AudioBundle {
        source: sound_effect,
        settings: PlaybackSettings::ONCE,
    });
}

pub fn despawn_paused_screen(
    mut commands: Commands,
    query: Query<Entity, With<PausedMenu>>,
    asset_server: Res<AssetServer>,
) {
    if let Ok(entity) = query.get_single() {
        commands.entity(entity).despawn_recursive();
        let sound_effect = asset_server.load("audio/confirmation_002.ogg");
        commands.spawn(AudioBundle {
            source: sound_effect,
            settings: PlaybackSettings::ONCE,
        });
    }
}

pub fn interact_with_resume_game_button(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ResumeGameButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match interaction {
            Interaction::Pressed => {
                *background_color = Color::rgba(0.0, 0.5, 0.0, 1.0).into();
                app_state_next_state.set(AppState::Playing);
            }
            Interaction::Hovered => {
                *background_color = Color::rgba(0.0, 0.5, 0.0, 0.75).into();
            }
            Interaction::None => {
                *background_color = Color::rgba(0.0, 0.5, 0.0, 0.5).into();
            }
        }
    }
}

pub fn interact_with_quit_game_button() {}
