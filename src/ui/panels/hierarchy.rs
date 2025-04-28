use bevy_egui::{egui, EguiContexts};
use bevy::{prelude::*, window::PrimaryWindow};
use crate::ui::events::CreateEntity;

pub fn manage_hierarchy_panels(
    mut contexts: EguiContexts,
    mut event: EventWriter<CreateEntity>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    all_entities: Query<(Entity, &Name), With<Transform>>
) {
    if let Ok(_) = window_query.single() {
        let ctx = contexts.ctx_mut();

        egui::SidePanel::left("Hierarchy")
            .resizable(true)
            .show(ctx, |ui| {

                ui.horizontal(|ui| {

                    ui.heading("Hierarchy");
                    ui.with_layout(
                        egui::Layout::right_to_left(egui::Align::RIGHT),
                        |ui| {
                            ui.menu_button("Actions", |ui| {
                                ui.menu_button("Objects", |ui| {

                                    if ui.button("Circle").clicked() {
                                        event.write(CreateEntity::Circle);
                                    }

                                    if ui.button("Regular Poly").clicked() {
                                        event.write(CreateEntity::ConvexPolygon);
                                    }

                                    if ui.button("Box").clicked() {
                                        event.write(CreateEntity::Rectangle);
                                    }

                                });
                            });
                        }
                    );
                });

                ui.separator();
                
                ui.vertical(|ui| {
                    for (i, (entity, name)) in all_entities.iter().enumerate() {
                        ui.collapsing(format!("{} {}", name, i), |ui| {
                            ui.horizontal(|ui| {
                                ui.label(format!("Id: {:?}", entity))
                            });
                        }); 
                    }
                });

                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
                ui.vertical(|ui| {
                    ui.strong("List of objects will go here!");
                });

            })
        ;
    }
}

