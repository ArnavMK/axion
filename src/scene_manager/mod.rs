use bevy::prelude::*;
use spawner::*;
use selection::*;

pub mod spawner;
pub mod selection;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app 
            .add_event::<SelectedEntityChanged>()
            .init_resource::<SelectedEntity>()
            .add_systems(Update, handle_entity_spawning)
        ;
    }
} 