use bevy_egui::egui::{self, Align};

pub fn manage_inspector_panel(ctx: &mut egui::Context) -> f32 {

    egui::SidePanel::right("Inspector")
        .resizable(true)
        .show(ctx, |ui| {
            
            ui.heading("Inspector");
        })
    .response.rect.width()
}
