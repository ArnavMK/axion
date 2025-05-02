use bevy_egui::{EguiContextPass, EguiPlugin};
use events::*;
use bevy::prelude::*;
use panels::{camera_hud::handle_camera_controller_hud, hierarchy::*};
use buffers::*;
use panels::inspector::*;

pub mod panels;
pub mod events;
pub mod buffers;
pub struct AxionUi;

impl Plugin for AxionUi {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<ComponentTextBuffers>()
            .add_plugins(EguiPlugin {
                enable_multipass_for_primary_context: true
            })
            .add_plugins(UiEvents)
            .add_systems(EguiContextPass, (
                manage_inspector_panel,
                manage_hierarchy_panels,
                handle_camera_controller_hud
            ))
        ;
    }
}
