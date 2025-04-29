use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct SelectedEntity {
    pub entity: Option<Entity>
}

impl Default for SelectedEntity {
    fn default() -> Self {
        Self { entity: None }
    }
}

impl SelectedEntity {

    pub fn set(&mut self, entity: Entity) {
        self.entity = Some(entity);
    }

    pub fn get(&self) -> Option<Entity> {
        self.entity
    }
}

#[derive(Event)]
pub struct SelectedEntityChanged {
    pub previous: Option<Entity>,
    pub current: Option<Entity>
}

#[derive(Component)]
pub struct SelectedEntityMarkerComponent;

pub fn attach_seelcted_entity_marker_component(
    mut commands: Commands,
    mut event: EventReader<SelectedEntityChanged>
) {
    for e in event.read() {
        if let Some(entity) = e.previous {
            println!("Removed marker from {:?}", entity);
            commands.entity(entity).remove::<SelectedEntityMarkerComponent>();
        }

        if let Some(entity) = e.current {
            println!("Added marker to {:?}", entity);
            commands.entity(entity).insert(SelectedEntityMarkerComponent);
        }
    }
}

