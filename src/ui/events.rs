use bevy::prelude::*;

#[derive(Event)]
pub enum CreateEntity {
    Circle,
    Triangle,
    Rectangle,
    Hexagon
}

pub struct UiEvents;

impl Plugin for UiEvents {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreateEntity>()
        ;
    }
} 

