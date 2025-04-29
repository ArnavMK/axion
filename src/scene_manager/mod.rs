use bevy::{prelude::*, render};
use spawner::*;
use selection::*;
use gizmos::*;

pub mod spawner;
pub mod selection;
pub mod gizmos;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_event::<SelectedEntityChanged>()
            .init_resource::<SelectedEntity>()
            .init_gizmo_group::<GridGizmoGroup>()
            .init_gizmo_group::<OverlayGizmoGroup>()
            .add_systems(Startup, gizmo_config_setup)
            .add_systems(Update, (
                attach_seelcted_entity_marker_component,
                render_grid,
                handle_entity_spawning
            ))
        ;
    }
} 
