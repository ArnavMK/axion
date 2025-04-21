use bevy_egui::{EguiContextPass, EguiPlugin};
use events::*;
use bevy::prelude::*;
use panels::hierarchy::*;
use panels::inspector::*;

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
            .add_systems(EguiContextPass, (manage_inspector_panel, manage_hierarchy_panels))
        ;
    }
}


    