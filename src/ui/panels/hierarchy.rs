use bevy_egui::{egui, EguiContexts};
use bevy::{prelude::*, window::PrimaryWindow};
use crate::ui::events::{CreateEntity, RemoveEntity};
use crate::scene_manager::selection::*;

pub fn manage_hierarchy_panels(
    mut contexts: EguiContexts,
    mut entity_creation_events: EventWriter<CreateEntity>,
    mut entity_removal_events: EventWriter<RemoveEntity>,
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
                                        entity_creation_events.write(CreateEntity::Circle);     
                                    }

                                    if ui.button("Regular Poly").clicked() {
                                        entity_creation_events.write(CreateEntity::ConvexPolygon);
                                    }

                                    if ui.button("Box").clicked() {
                                        entity_creation_events.write(CreateEntity::Rectangle);
                                    }

                                });
                            });
                        }
                    );
                });

                ui.separator();
                
                ui.vertical(|ui| {
                    for (i, (entity, name)) in all_entities.iter().enumerate() {
                        let entity_collapsable = ui.collapsing(format!("{} {}", name, i), |ui| {
                            ui.label(format!("Id: {:?}", entity));
                        }).header_response;
                            
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

                            if ui.button("delete").clicked() {
                                entity_removal_events.write(RemoveEntity {
                                    target: entity
                                });
                                ui.close_menu();
                            }
                            
                            if ui.button("Close").clicked() {
                                ui.close_menu();
                            }

                        });
                    }
                });

                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

            })
        ;
    }
}

