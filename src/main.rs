use crate::ui::AxionUi;
use bevy::prelude::*;

pub mod ui;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.25, 0.25, 0.25)))
        .add_plugins(DefaultPlugins)
        .add_plugins(AxionUi)
        .add_systems(Startup, setup_system)
        .run();
}


fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Circle
    commands.spawn((
        Mesh2d(meshes.add(Circle::new(50.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.1, 0.0)))),
        Transform::from_translation(Vec3::new(-150., 0., 0.)),
    ));

    // Rectangles
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50., 100.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.4, 0.3)))),
        Transform::from_translation(Vec3::new(-50., 0., 0.)),
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50., 100.))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.4, 0.3)))),
        Transform::from_translation(Vec3::new(50., 0., 0.)),
    ));

    // Hexagon
    commands.spawn((
        Mesh2d(meshes.add(RegularPolygon::new(50., 6))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.8, 0.7, 0.6)))),
        Transform::from_translation(Vec3::new(150., 0., 0.)),
    ));

    commands.spawn(Camera2d);
}
