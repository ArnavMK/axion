use bevy_egui::egui;
use bevy::prelude::*;
use crate::physics::{collider::*, shapes::*};
use crate::ui::{buffers::*, ComponentTextBuffers};

pub trait ComponentUi {
    fn ui(&mut self, ui: &mut egui::Ui, buffer_map: &mut ResMut<ComponentTextBuffers>, entity: Entity);
}

impl ComponentUi for Transform {

    fn ui(&mut self, ui: &mut egui::Ui, buffer_map: &mut ResMut<ComponentTextBuffers>, entity: Entity) {

        let buffer = buffer_map.transform_buff
            .entry(entity)
            .or_insert_with(|| TransformBuffer {
                pos_x: self.translation.x.to_string(), pos_y: self.translation.y.to_string(),
                rot_x: self.rotation.x.to_string(), rot_y: self.rotation.y.to_string(),
                scale_factor: "1".to_string()
            });
            

        let field_width = ui.available_width()/15.0;
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
                    ui.add_space(field_width + 1.0);

                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.scale_factor));
                });
                ui.end_row();

                let something = 10.0;
                println!("{}", something);

                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        if let Ok(pos_x) = buffer.pos_x.parse::<f32>() {
                            self.translation.x = pos_x;
                        }
                        if let Ok(pos_y) = buffer.pos_y.parse::<f32>() {
                            self.translation.y = pos_y;
                        }
                        if let Ok(rot_x) = buffer.rot_x.parse::<f32>() {
                            self.rotation.x = rot_x;
                        }
                        if let Ok(pos_y) = buffer.pos_y.parse::<f32>() {
                            self.rotation.y = pos_y;
                        }
                        if let Ok(factor) = buffer.scale_factor.parse::<f32>() {
                            self.scale *= factor;
                        }
                    }
                });
            });
        });
        ui.separator();
    }
}

impl ComponentUi for Collider<CircleShape>{
    fn ui(&mut self, ui: &mut egui::Ui, buffer_map: &mut ResMut<ComponentTextBuffers>, entity: Entity) {

        let buffer = buffer_map.circle_collider_buff.entry(entity)
            .or_insert_with(|| {
                CircleColliderBuffer { radius: self.shape.radius.to_string()}
            });

        let field_width = ui.available_width()/10.0;
        ui.collapsing("Circle Collider", |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.strong("Radius");
                    ui.add_space(field_width + 2.0);
                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.radius));
                });
            });
        });

        ui.separator();
    }
}

impl ComponentUi for Collider<ConvexPolygonShape>{
    fn ui(&mut self, ui: &mut egui::Ui, buffer_map: &mut ResMut<ComponentTextBuffers>, entity: Entity) {

        let buffer = buffer_map.polygon_collider_buff.entry(entity)
            .or_insert_with(|| {
                ConvexPolygonBuffer {
                    circum_radius: self.shape.circum_radius.to_string(),
                    sides: self.shape.sides.to_string()
                }
            });

        let field_width = ui.available_width()/10.0;
        ui.collapsing("Polygon Collider", |ui| {
            ui.vertical(|ui| {

                ui.horizontal(|ui| {
                    ui.strong("Radius");
                    ui.add_space(field_width + 2.0);
                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.circum_radius));
                });

                ui.horizontal(|ui| {
                    ui.strong("Side Number");
                    ui.add_space(field_width + 2.0);
                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.sides));
                });
            });
        });

        ui.separator();
    }
}

impl ComponentUi for Collider<RectangleShape>{
    fn ui(&mut self, ui: &mut egui::Ui, buffer_map: &mut ResMut<ComponentTextBuffers>, entity: Entity) {

        let buffer = buffer_map.rectangle_collider_buff.entry(entity)
            .or_insert_with(|| {
                RectangleColliderBuffer {
                    width: self.shape.width.to_string(),
                    height: self.shape.height.to_string()
                }
            });

        let field_width = ui.available_width()/10.0;
        ui.collapsing("Polygon Collider", |ui| {
            ui.vertical(|ui| {

                ui.horizontal(|ui| {
                    ui.strong("Width");
                    ui.add_space(field_width);
                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.width));
                });

                ui.horizontal(|ui| {
                    ui.strong("Height");
                    ui.add_space(field_width - 1.0);
                    ui.add_sized([60.0, 20.0], egui::TextEdit::singleline(&mut buffer.height));
                });
            });
        });

        ui.separator();
    }
}

