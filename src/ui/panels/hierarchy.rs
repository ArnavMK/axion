use bevy_egui::egui;

pub fn manage_hierarchy_panels(ctx: &mut egui::Context) -> f32 {
    
    if ctx.input(|i| i.screen_rect.size().x <= 1.0 || i.screen_rect.size().y <= 1.0) {
        return 0.0;
    }

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
                                    println!("Create a circle later");
                                }

                                if ui.button("Triangle").clicked() {
                                    println!("Create a triganle later");
                                }
                            });

                            ui.menu_button("Pyhsics", |ui| {
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

            ui.separator();

            // Rest of your panel content...
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());

            ui.vertical(|ui| {
                ui.strong("List of objects will go here!");
            });

        })
    .response.rect.width()

}

