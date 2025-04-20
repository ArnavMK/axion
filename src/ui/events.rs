use bevy::prelude::*;

#[derive(Event)]
pub enum CreateEntity {
    Circle,
    Rectangle,
    ConvexPolygon 
}

pub struct UiEvents;

impl Plugin for UiEvents {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreateEntity>()
        ;
    }
} 

