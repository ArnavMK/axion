use bevy_egui::{egui, EguiContexts};
use bevy::{prelude::*, window::PrimaryWindow};
use crate::scene_manager::camera_controller::*;

pub fn handle_camera_controller_hud(
    mut contexts: EguiContexts,
    mut camera_controller_mode: ResMut<CameraControllerState>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {
    if let Ok(_) = window_query.single() {
        let ctx = contexts.ctx_mut(); 

        egui::Window::new("Camera Controller")
            .resizable(false)
            .title_bar(false)
            .anchor(egui::Align2::CENTER_TOP, egui::Vec2::new(0.0, -5.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {

                    if ui.button("▣").clicked() {
                        camera_controller_mode.set(CameraControllerMode::General);
                    }

                    if ui.button("✋").clicked() {
                        camera_controller_mode.set(CameraControllerMode::Pan);
                    }

                    if ui.button("🖊").clicked() {
                        camera_controller_mode.set(CameraControllerMode::Picker);
                    }
                });
            })
        ;
    }
}

