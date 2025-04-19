use bevy::{prelude::*, render::camera::Viewport, window::PrimaryWindow};
use bevy_egui::{EguiContextPass, EguiContexts, EguiPlugin};
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
            // .add_systems(EguiContextPass, ui_viewport_manager)
            .add_systems(EguiContextPass, ui_viewport_manager)
        ;
    }
}

fn ui_viewport_manager(
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera>,
    window: Single<&Window, With<PrimaryWindow>>,
) {

    let ctx = contexts.ctx_mut();

    let max_panel_width = window.physical_width() as f32 * 0.5;

    let hierarchy_width = manage_hierarchy_panels(ctx) * window.scale_factor();
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
    