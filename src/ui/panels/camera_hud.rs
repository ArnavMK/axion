use bevy_egui::{egui, EguiContexts};
use bevy::prelude::*;
use crate::scene_manager::camera_controller::*;

pub fn handle_camera_controller_hud(
    mut contexts: EguiContexts,
    mut camera_controller_mode: ResMut<CameraControllerState>
) {
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

