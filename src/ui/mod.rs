use bevy_egui::{EguiContextPass, EguiPlugin};
use std::collections::HashMap;
use events::*;
use bevy::prelude::*;
use panels::hierarchy::*;
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
            .add_systems(EguiContextPass, (manage_inspector_panel, manage_hierarchy_panels))
        ;
    }
}

#[derive(Resource)]
pub struct ComponentTextBuffers {
    pub transform_buff: HashMap<Entity, TransformBuffer> 
}

impl Default for ComponentTextBuffers {
    fn default() -> Self {
        Self {
            transform_buff: HashMap::new()
        }
    }
}