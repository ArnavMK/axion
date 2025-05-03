use bevy::{input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll, MouseWheel}, prelude::*};
use crate::CameraSettings;

#[derive(Default)]
pub enum CameraControllerMode {
    #[default]
    General,
    Picker,
    Pan,
}

#[derive(Resource, Default)]
pub struct CameraControllerState {
    pub state: CameraControllerMode 
}

impl CameraControllerState {
    pub fn set(&mut self, state: CameraControllerMode) {
        self.state = state 
    }

    pub fn get(&self) -> &CameraControllerMode {
        &self.state
    }
}

pub fn handle_camera(
    mouse_motions: Res<AccumulatedMouseMotion>,
    mouse_inputs: Res<ButtonInput<MouseButton>>,
    mut mouse_scroll: EventReader<MouseWheel>,
    camera_state: Res<CameraControllerState>,
    mut camera_transform: Single<&mut Transform, With<Camera2d>>,
    camera_projection: Single<&mut Projection, With<Camera2d>>,
    camera_settings: Res<CameraSettings>,
    time: Res<Time>,
) {
    match camera_state.get() {
        CameraControllerMode::General => {

            // panning
            if mouse_motions.delta != Vec2::ZERO && mouse_inputs.pressed(MouseButton::Middle) {
                let new_position = -1.0 * mouse_motions.delta * camera_settings.pan_speed * time.delta_secs();
                camera_transform.translation.x += new_position.x;
                camera_transform.translation.y -= new_position.y;
            }
            
            // zooming 
            match *camera_projection.into_inner() {
                Projection::Orthographic(ref mut ortho) => {
                    for scroll in mouse_scroll.read() {
                        let zoom_factor = 1.0 + -scroll.y * camera_settings.zoom_speed * time.delta_secs();
                        ortho.scale = (ortho.scale * zoom_factor).clamp(
                            camera_settings.zoom_range.start, 
                            camera_settings.zoom_range.end
                        );
                    }
                }
                _ => ()
            }
            
            // selection (later on)
            if mouse_inputs.pressed(MouseButton::Left) {
                println!("Select entity under mouse");
            }
        }
        CameraControllerMode::Picker => println!("Picker mode"), 
        CameraControllerMode::Pan => println!("Pan mode")
    }

}
