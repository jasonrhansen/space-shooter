use super::*;
use super::{components::*, resources::UiAssets};
use crate::{health::Health, player::components::Player, score::resources::Score};
use bevy::app::AppExit;
use bevy::sprite::Anchor;
use bevy_kira_audio::prelude::*;

pub fn setup(
    mut commands: Commands,
    score_text: Query<Entity, With<ScoreText>>,
    health_text: Query<Entity, With<HealthText>>,
) {
    if score_text.get_single().is_err() {
        commands.spawn((
            Text2d::new("Score: 0"),
            TextFont::from_font_size(32.0),
            TextLayout::new_with_justify(JustifyText::Right),
            Anchor::BottomRight,
            Transform::from_translation(Vec3::new(5.0, 5.0, 0.0)),
            ScoreText,
        ));
    }

    if health_text.get_single().is_err() {
        commands.spawn((
            Text2d::new("Health: 100%"),
            TextFont::from_font_size(32.0),
            TextLayout::new_with_justify(JustifyText::Left),
            Anchor::BottomLeft,
            Transform::from_translation(Vec3::new(5.0, 5.0, 0.0)),
            HealthText,
        ));
    }
}

pub fn update_health_text(
    mut health_text: Query<&mut Text, With<HealthText>>,
    player_health: Query<&Health, With<Player>>,
) {
    if let Ok(health) = player_health.get_single() {
        if let Ok(mut text) = health_text.get_single_mut() {
            text.0 = format!("Health: {}%", health.percent);
        }
    }
}

pub fn update_score_text(mut query: Query<&mut Text, With<ScoreText>>, score: Res<Score>) {
    if score.is_changed() {
        if let Ok(mut text) = query.get_single_mut() {
            text.0 = format!("Score: {}", score.value);
        }
    }
}

pub fn spawn_paused_screen(mut commands: Commands, ui_assets: Res<UiAssets>, audio: Res<Audio>) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(15.0),
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5).into()),
            PausedMenu,
        ))
        .with_children(|parent| {
            parent.spawn((Text2d::new("Paused"), TextFont::from_font_size(100.0)));
            parent
                .spawn((
                    Button {},
                    BackgroundColor(Color::srgba(0.0, 0.5, 0.0, 0.5).into()),
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(MENU_BUTTON_WIDTH),
                        height: Val::Px(MENU_BUTTON_HEIGHT),
                        ..default()
                    },
                    ResumeGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn((Text2d::new("Resume Game"), TextFont::from_font_size(50.0)));
                });

            parent
                .spawn((
                    Button {},
                    BackgroundColor(Color::srgba(0.5, 0.5, 0.0, 0.5).into()),
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(MENU_BUTTON_WIDTH),
                        height: Val::Px(MENU_BUTTON_HEIGHT),
                        ..default()
                    },
                    NewGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn((Text2d::new("New Game"), TextFont::from_font_size(50.0)));
                });

            parent
                .spawn((
                    Button {},
                    BackgroundColor(Color::srgba(0.5, 0.0, 0.0, 0.5).into()),
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(MENU_BUTTON_WIDTH),
                        height: Val::Px(MENU_BUTTON_HEIGHT),
                        ..default()
                    },
                    QuitGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn((Text2d::new("Quit"), TextFont::from_font_size(50.0)));
                });
        });

    audio.play(ui_assets.pause_sound.clone());
}

pub fn despawn_paused_screen(
    mut commands: Commands,
    paused_manu: Query<Entity, With<PausedMenu>>,
    ui_assets: Res<UiAssets>,
    audio: Res<Audio>,
) {
    if let Ok(entity) = paused_manu.get_single() {
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

        background_color.0.set_alpha(alpha);
    }
}

pub fn resume_game_button_action(
    mut buttons: Query<&Interaction, (Changed<Interaction>, With<ResumeGameButton>)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    ui_assets: Res<UiAssets>,
    audio: Res<Audio>,
) {
    if let Ok(interaction) = buttons.get_single_mut() {
        if *interaction == Interaction::Pressed {
            next_game_state.set(GameState::Playing);
            audio.play(ui_assets.resume_game_sound.clone());
        }
    }
}

pub fn new_game_button_action(
    mut buttons: Query<&Interaction, (Changed<Interaction>, With<NewGameButton>)>,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    if let Ok(interaction) = buttons.get_single_mut() {
        if *interaction == Interaction::Pressed {
            next_app_state.set(AppState::Loading);
            next_game_state.set(GameState::Playing);
        }
    }
}

pub fn quit_game_button_action(
    mut buttons: Query<&Interaction, (Changed<Interaction>, With<QuitGameButton>)>,
    mut exit_writer: EventWriter<AppExit>,
) {
    if let Ok(interaction) = buttons.get_single_mut() {
        if *interaction == Interaction::Pressed {
            exit_writer.send(AppExit::Success);
        }
    }
}

pub fn spawn_game_over_screen(mut commands: Commands) {
    commands
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                row_gap: Val::Px(15.0),
                position_type: PositionType::Absolute,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5).into()),
            GameOverMenu,
        ))
        .with_children(|parent| {
            parent.spawn((Text2d::new("Game Over!"), TextFont::from_font_size(100.0)));
            parent
                .spawn((
                    Button {},
                    BackgroundColor(Color::srgba(0.0, 0.5, 0.0, 0.5).into()),
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(MENU_BUTTON_WIDTH),
                        height: Val::Px(MENU_BUTTON_HEIGHT),
                        ..default()
                    },
                    NewGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn((Text2d::new("New Game"), TextFont::from_font_size(50.0)));
                });

            parent
                .spawn((
                    Button {},
                    BackgroundColor(Color::srgba(0.5, 0.0, 0.0, 0.5).into()),
                    Node {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        width: Val::Px(MENU_BUTTON_WIDTH),
                        height: Val::Px(MENU_BUTTON_HEIGHT),
                        ..default()
                    },
                    QuitGameButton,
                ))
                .with_children(|parent| {
                    parent.spawn((Text::new("Quit"), TextFont::from_font_size(50.0)));
                });
        });
}

pub fn despawn_game_over_screen(
    mut commands: Commands,
    game_over_menu: Query<Entity, With<GameOverMenu>>,
) {
    if let Ok(entity) = game_over_menu.get_single() {
        commands.entity(entity).despawn_recursive();
    }
}
