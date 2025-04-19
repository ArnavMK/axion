use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};
use bevy_egui::{egui, EguiContextPass, EguiContexts, EguiPlugin};
use crate::ui::AxionUi;

pub mod ui;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.25, 0.25, 0.25)))
        .add_plugins(DefaultPlugins)
        .add_plugins(AxionUi)
        .add_systems(Startup, setup_system)
        .run();
}

fn ui_example_system(
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera>,
    window: Single<&mut Window, With<PrimaryWindow>>,
) {
    // egui context
    let ctx = contexts.ctx_mut();

    let mut left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width(); // height is ignored, as the panel has a hight of 100% of the screen

    let mut right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width(); // height is ignored, as the panel has a height of 100% of the screen

    let mut top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Top resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height(); // width is ignored, as the panel has a width of 100% of the screen
    let mut bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Bottom resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height(); // width is ignored, as the panel has a width of 100% of the screen

    // Scale from logical units to physical units
    left *= window.scale_factor();
    right *= window.scale_factor();
    top *= window.scale_factor();
    bottom *= window.scale_factor();

    let pos = UVec2::new(left as u32, top as u32);
    let size = UVec2::new(window.physical_width(), window.physical_height())
        - pos
        - UVec2::new(right as u32, bottom as u32);

    camera.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: size,
        ..default()
    });
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