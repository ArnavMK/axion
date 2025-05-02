use bevy::prelude::*;
use spawner::*;
use selection::*;
use gizmos::*;
use camera_controller::*;

pub mod spawner;
pub mod selection;
pub mod gizmos;
pub mod camera_controller;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_event::<SelectedEntityChanged>()
            .init_resource::<SelectedEntity>()
            .init_resource::<CameraControllerState>()
            .init_gizmo_group::<GridGizmoGroup>()
            .init_gizmo_group::<OverlayGizmoGroup>()
            .add_systems(Startup, gizmo_config_setup)
            .add_systems(Update, (
                attach_seelcted_entity_marker_component,
                render_grid,
                handle_entity_spawning,
                handle_entity_despawning,
                handle_camera
            ))
        ;
    }
} 
