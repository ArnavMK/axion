use bevy_egui::{egui, EguiContexts};
use bevy::{prelude::*, window::PrimaryWindow};
use crate::ui::events::CreateEntity;
use crate::scene_manager::selection::*;

pub fn manage_hierarchy_panels(
    mut contexts: EguiContexts,
    mut event: EventWriter<CreateEntity>,
    mut selection_events: EventWriter<SelectedEntityChanged>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    all_entities: Query<(Entity, &Name), With<Transform>>,
    mut selected_entity: ResMut<SelectedEntity>
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
                        let entity_collapsable = ui.collapsing(format!("{} {}", name, i), |_| {}).header_response;

                        // entity selection
                        if entity_collapsable.clicked() {
                            let previous = selected_entity.get();
                            selected_entity.set(entity);

                            selection_events.write(SelectedEntityChanged {
                                previous,
                                current: Some(entity),
                            });
                        }
                        
                        // a context menu belongin to the current entity
                        entity_collapsable.context_menu(|ui| {
                            if ui.button("Close").clicked() {
                                ui.close_menu();
                            }

                            if ui.button("delete").clicked() {
                                println!("Delete this entity please");
                                ui.close_menu();
                            }
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

