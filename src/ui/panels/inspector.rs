use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, Align}, EguiContexts};
use super::component_ui::*;
use crate::{physics::shapes::*, ui::ComponentTextBuffers, physics::collider::*};
use crate::scene_manager::selection::*;


pub fn manage_inspector_panel(
    mut contexts: EguiContexts,
    window_query: Query<&Window, With<PrimaryWindow>>,
    selected_entity_resource: Res<SelectedEntity>,
    mut input_buffers: ResMut<ComponentTextBuffers>,
    mut transforms: Query<&mut Transform>,
    mut circle_colliders: Query<&mut Collider<CircleShape>>,
    mut polygon_colliders: Query<&mut Collider<ConvexPolygonShape>>,
    mut rectangle_colliders: Query<&mut Collider<RectangleShape>>
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

                        if let Ok(mut transform) = transforms.get_mut(entity) {
                            transform.ui(ui, &mut input_buffers, entity);
                        }

                        if let Ok(mut collider) = circle_colliders.get_mut(entity) {
                            collider.ui(ui, &mut input_buffers, entity);
                        }

                        if let Ok(mut collider) = polygon_colliders.get_mut(entity) {
                            collider.ui(ui, &mut input_buffers, entity);
                        }

                        if let Ok(mut collider) = rectangle_colliders.get_mut(entity) {
                            collider.ui(ui, &mut input_buffers, entity);
                        }
                        
                        ui.add_space(10.0);
                        
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
