use crate::ui::AxionUi;
use crate::scene_manager::*;
use bevy::prelude::*;

pub mod ui;
pub mod physics;
pub mod scene_manager;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.25, 0.25, 0.25)))
        .add_systems(Startup, setup_system)
        .add_plugins((DefaultPlugins, AxionUi, SceneManagerPlugin))
    .run();
}

fn setup_system(mut commands: Commands) {
    commands.spawn(Camera2d);
}
