use bevy_egui::egui;
use bevy::prelude::*;
use crate::ui::{buffers::TransformBuffer, ComponentTextBuffers};

pub trait ComponentUi {
    fn ui(&self, ui: &mut egui::Ui, buffer_map: &mut ResMut<ComponentTextBuffers>, entity: Entity);
}

impl ComponentUi for Transform {

    fn ui(&self, ui: &mut egui::Ui, buffer_map: &mut ResMut<ComponentTextBuffers>, entity: Entity) {

        let buffer = buffer_map.transform_buff
            .entry(entity)
            .or_insert_with(|| TransformBuffer {
                pos_x: self.translation.x.to_string(), pos_y: self.translation.y.to_string(),
                rot_x: self.rotation.x.to_string(), rot_y: self.rotation.y.to_string(),
                scale_factor: "1".to_string()
            });
            

        let field_width = ui.available_width()/10.0;
        ui.collapsing("Transform", |ui| {
            egui::Grid::new("transform_grid")
            .show(ui, |ui| {

                ui.horizontal(|ui| {
                    ui.strong("Position:");
                    ui.add_space(field_width);

                    ui.label("x:");
                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.pos_x));

                    ui.add_space(field_width);
                    ui.label("y:");
                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.pos_y));
                });
                ui.end_row();

                ui.horizontal(|ui| {
                    ui.strong("Rotation");
                    ui.add_space(field_width);

                    ui.label("x:");
                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.rot_x));

                    ui.add_space(field_width);
                    ui.label("y:");
                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.rot_y));
                });
                ui.end_row();
                
                ui.horizontal(|ui| {
                    ui.strong("ScaleFactor");
                    ui.add_space(field_width + 2.0);

                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.scale_factor));
                });
                ui.end_row();

                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        // save transform information
                    }
                });
            });
        });
        ui.separator();
    }
}