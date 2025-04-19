use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};
use bevy_egui::{egui, EguiContextPass, EguiContexts, EguiPlugin};

pub mod panels;
pub struct AxionUi;

impl Plugin for AxionUi {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin {
                enable_multipass_for_primary_context: true
            })
            .add_systems(EguiContextPass, ui_viewport_manager)
        ;
    }
}


fn ui_viewport_manager(
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera>,
    window: Single<&mut Window, With<PrimaryWindow>>,
) {

    let ctx = contexts.ctx_mut();

    let max_panel_width = window.physical_width() as f32 * 0.5;


    let hierarchy_width = manage_hierarcchy_panel(ctx) * window.scale_factor();
    let inspector_width = manage_inspector_panel(ctx) * window.scale_factor();

    let hierarchy_width = hierarchy_width.clamp(5.0, max_panel_width);
    let inspector_width = inspector_width.clamp(5.0, max_panel_width);

    let pos = UVec2::new(hierarchy_width as u32, 0);
    let size = UVec2::new(window.physical_width(), window.physical_height()) 
        - pos
        - UVec2::new(inspector_width as u32, 0);

    camera.viewport = Some(Viewport {
        physical_position: pos,
        physical_size: size,
        ..default()
    });
    
}
    
fn manage_hierarcchy_panel(ctx: &mut egui::Context) -> f32 {
    
    egui::SidePanel::left("Hierarchy")
        .resizable(true)
        .show(ctx, |ui| {
            // Top row with heading and right-aligned button
            ui.horizontal(|ui| {
                // Left-aligned heading
                ui.heading("Hierarchy");
                
                // Right-aligned menu button
                ui.with_layout(
                    egui::Layout::right_to_left(egui::Align::RIGHT),
                    |ui| {
                        ui.menu_button("Actions", |ui| {
                            ui.menu_button("Objects", |ui| {
                                if ui.button("Circle").clicked() {
                                    println!("Create a circle later");
                                }

                                if ui.button("Triangle").clicked() {
                                    println!("Create a triganle later");
                                }
                            });
                        });
                    }
                );
            });

            // Rest of your panel content...
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

        })
    .response.rect.width()

}

fn manage_inspector_panel(ctx: &mut egui::Context) -> f32 {

    egui::SidePanel::right("Inspector")
        .resizable(true)
        .show(ctx, |ui| {
            
            ui.heading("Inspector");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
            
        })
        .response.rect.width()
}