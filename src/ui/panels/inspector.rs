use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui::{self, Align}, EguiContexts};
use crate::scene_manager::selection::*;

pub fn manage_inspector_panel(
    selected_entity: Res<SelectedEntity>,
    mut contexts: EguiContexts,
    window_query: Query<&Window, With<PrimaryWindow>>

) {

    if let Ok(_) = window_query.single() {
        let ctx = contexts.ctx_mut();
        egui::SidePanel::right("Inspector")
            .resizable(true)
            .show(ctx, |ui| {
                
                ui.heading("Inspector");
                
                ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

                ui.with_layout(
                    egui::Layout::bottom_up(Align::Center),
                    |ui| {
                        if ui.button("Add Component").clicked() {
                            println!("Component will be added later. (Maybe never)");
                        }
                    }
                );

            })
        ;
    }
}
