use bevy::prelude::*;

#[derive(Event)]
pub enum CreateEntity {
    Circle,
    Rectangle,
    ConvexPolygon 
}

#[derive(Event)]
pub struct RemoveEntity {
    pub target: Entity
}

pub struct UiEvents;

impl Plugin for UiEvents {
    fn build(&self, app: &mut App) {
        app
            .add_event::<CreateEntity>()
            .add_event::<RemoveEntity>()
        ;
    }
} 

