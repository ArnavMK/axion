use bevy_egui::egui::{self, Align};

pub fn manage_inspector_panel(ctx: &mut egui::Context) -> f32 {

    if ctx.input(|i| i.screen_rect.size().x <= 1.0 || i.screen_rect.size().y <= 1.0) {
        return 0.0;
    }

    egui::SidePanel::right("Inspector")
        .resizable(true)
        .show(ctx, |ui| {
            
            ui.heading("Inspector");

            ui.with_layout(
                egui::Layout::bottom_up(Align::Center),
                |ui| {
                    if ui.button("Add Component").clicked() {
                        println!("Component will be added later. (Maybe never)");
                    }
                }
            )
            
        })
    .response.rect.width()
}
