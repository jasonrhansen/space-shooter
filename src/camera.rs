use crate::{VIEWPORT_HEIGHT, VIEWPORT_WIDTH};
use bevy::{prelude::*, render::camera::ScalingMode, window::PrimaryWindow};

#[derive(Component)]
pub struct GameCamera;

pub fn spawn_camera(mut commands: Commands, window: Query<&Window, With<PrimaryWindow>>) {
    let window = window.get_single().unwrap();
    commands.spawn((
        GameCamera,
        Camera2d::default(),
        Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
        OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: VIEWPORT_WIDTH,
                min_height: VIEWPORT_HEIGHT,
            },
            ..OrthographicProjection::default_2d()
        },
    ));
}
