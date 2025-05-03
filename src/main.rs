use crate::ui::AxionUi;
use std::ops::Range;
use crate::scene_manager::*;
use bevy::render::camera::ScalingMode;
use bevy::prelude::*;

pub mod ui;
pub mod physics;
pub mod scene_manager;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.25, 0.25, 0.25)))
        .init_resource::<CameraSettings>()
        .add_systems(Startup, setup_system)
        .add_plugins((DefaultPlugins, AxionUi, SceneManagerPlugin))
    .run();
}

#[derive(Resource)]
pub struct CameraSettings {
    pub orthographic_viewport_height: f32,
    pub zoom_range: Range<f32>,
    pub pan_speed: f32,
    pub zoom_speed: f32,
}

impl Default for CameraSettings {
    fn default() -> Self {
        Self {
            orthographic_viewport_height: 1000.0,
            zoom_range: 0.1..10.0,
            pan_speed: 70.0,
            zoom_speed: 2.0,
        }
    }
}

fn setup_system(
    mut commands: Commands,
    camera_settings: Res<CameraSettings>
) {
    commands.spawn((
        Camera2d,
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: camera_settings.orthographic_viewport_height,
            },
            scale: 1.0,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
