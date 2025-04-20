use bevy::prelude::*;
use crate::physics::collider::*;

pub struct RectangleShape {
    pub width: f32,
    pub height: f32
}

impl ColliderShape for RectangleShape {
    fn vertices(&self) {
        println!("Vertices for rectangle");
    }

    fn distance_to_point(&self, _: Vec2) {
        println!("Distance to point for rectangle");
    }
}
