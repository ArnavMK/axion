use bevy::prelude::*;

pub trait ColliderShape {
    fn vertices(&self);
    fn distance_to_point(&self, point: Vec2);
}

#[derive(Component)]
pub struct Collider<T: ColliderShape>{
    pub shape: T
}

