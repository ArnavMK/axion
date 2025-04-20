use bevy::prelude::*;
use crate::physics::collider::*;

pub struct ConvexPolygonShape {
    pub circum_radius: f32,
    pub sides: u32 
}

impl ColliderShape for ConvexPolygonShape {
    fn vertices(&self) {
        println!("Vertices for ConvexPolygon");
    }

    fn distance_to_point(&self, _: Vec2) {
        println!("Distance to point for ConvexPolygon");
    }
}
