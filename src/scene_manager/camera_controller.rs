use bevy::{input::mouse::{AccumulatedMouseScroll, AccumulatedMouseMotion}, prelude::*};

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
    mouse_scroll: Res<AccumulatedMouseScroll>,
    camera_state: Res<CameraControllerState>,
    mut camera_transform: Single<&mut Transform, With<Camera2d>>,
    time: Res<Time>
) {
    match camera_state.get() {
        CameraControllerMode::General => {
            
            if mouse_motions.delta != Vec2::ZERO && mouse_inputs.pressed(MouseButton::Left) {
                let new_position = mouse_motions.delta * -1.0 * 200.0 * time.delta_secs();
                camera_transform.translation.x += new_position.x;
                camera_transform.translation.y += new_position.y;
            }
        }
        CameraControllerMode::Picker => println!("Picker mode"), 
        CameraControllerMode::Pan => println!("Pan mode")
    }

    if mouse_scroll.delta != Vec2::ZERO {
        println!("Scrolling with mouse");
    }
}
