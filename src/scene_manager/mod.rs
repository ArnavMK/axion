use bevy::prelude::*;
use spawner::*;

pub mod spawner;

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, handle_entity_spawning)
        ;
    }
} 