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