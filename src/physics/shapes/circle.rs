use bevy::prelude::*;
use crate::physics::collider::*;

pub struct CircleShape {
    pub radius: f32
}

impl ColliderShape for CircleShape{

    fn distance_to_point(&self, _: Vec2) {
        println!("Distance to point for circle shape");
    }

    fn vertices(&self) {
        println!("Vertices for a circle");
    }
}