use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, Align}, EguiContexts};
use crate::scene_manager::selection::*;

pub trait ComponentUi {
    fn ui(&self, ui: &mut egui::Ui);
}

impl ComponentUi for Transform {
    fn ui(&self, ui: &mut egui::Ui) {

        ui.collapsing("Transform", |ui| {
            ui.horizontal(|ui| {
                ui.strong("Position:");
                ui.add_space(30.0);

                ui.label("x:");
                ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut self.translation.x.to_string()));

                ui.label("y:");
                ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut self.translation.y.to_string()));
            });

            ui.horizontal(|ui| {
                ui.strong("Rotation");
                ui.add_space(30.0);

                ui.label("x:");
                ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut self.rotation.x.to_string()));

                ui.label("y:");
                ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut self.rotation.y.to_string()));
            });

            ui.horizontal(|ui| {
                ui.strong("ScaleFactor");
                ui.add_space(35.0);

                ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut "1".to_string()));
            });
        });
        ui.separator();
    }
}

pub fn manage_inspector_panel(
    mut contexts: EguiContexts,
    window_query: Query<&Window, With<PrimaryWindow>>,
    selected_entity_resource: Res<SelectedEntity>,
    transforms: Query<&Transform>
) {

    if let Ok(_) = window_query.single() {
        let ctx = contexts.ctx_mut();
        egui::SidePanel::right("Inspector")
            .resizable(true)
            .show(ctx, |ui| {
                
                ui.heading("Inspector");
                ui.separator();


                if let Some(entity) = selected_entity_resource.get() {
                    ui.vertical(|ui| {
                        if let Ok(transform) = transforms.get(entity) {
                            transform.ui(ui);
                        }

                        ui.add_space(20.0);
                        
                        ui.with_layout(
                            egui::Layout::top_down(Align::Center),
                            |ui| {
                                if ui.button("Add Component").clicked() {
                                    println!("Component will be added later. (Maybe never)");
                                }
                            }
                        );
                    });
                } 

                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            })
        ;
    }
}
