use bevy::{color::palettes::css::*, math::Isometry2d, prelude::*};

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct GridGizmoGroup;

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct OverlayGizmoGroup;

pub fn gizmo_config_setup(mut store: ResMut<GizmoConfigStore>) {
    let (grid_config, _) = store.config_mut::<GridGizmoGroup>();
    grid_config.depth_bias = -0.01; // Push it behind
    grid_config.line.width = 0.15;

    let (overlay_config, _) = store.config_mut::<OverlayGizmoGroup>();
    overlay_config.line.width = 1.0;
}

pub fn render_grid(mut gizmos: Gizmos<GridGizmoGroup>) {
    gizmos.grid_2d(
        Isometry2d::IDENTITY,
        UVec2::new(100, 100),
        Vec2::new(40.0, 40.0),
        LIGHT_GRAY,
    ).outer_edges();
}


