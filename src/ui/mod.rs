use bevy::window::PrimaryWindow;
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin};
use bevy::prelude::*;
use crate::ui::panels::hierarchy::*;
use crate::ui::panels::inspector::*;
use events::*;

pub mod panels;
pub mod events;
pub struct AxionUi;

impl Plugin for AxionUi {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(EguiPlugin {
                enable_multipass_for_primary_context: true
            })
            .add_plugins(UiEvents)
            .add_systems(EguiContextPass, ui_manager)
        ;
    }
}

fn ui_manager(
    mut contexts: EguiContexts,
    mut entity_creation_events: EventWriter<CreateEntity>,
    window_query: Query<&Window, With<PrimaryWindow>>
) {

    if let Ok(_) = window_query.single() {
        let ctx = contexts.ctx_mut();
        manage_hierarchy_panels(ctx, &mut entity_creation_events);
        manage_inspector_panel(ctx);
    }

}

    