use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin};
use bevy::prelude::*;
use crate::ui::panels::hierarchy::*;
use crate::ui::panels::inspector::*;

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

fn ui_viewport_manager(mut contexts: EguiContexts) {

    let ctx = contexts.ctx_mut();
    manage_hierarchy_panels(ctx);
    manage_inspector_panel(ctx);

}

    