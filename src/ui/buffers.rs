use bevy::prelude::*;
use std::collections::HashMap;

#[derive(Resource)]
pub struct ComponentTextBuffers {
    pub transform_buff: HashMap<Entity, TransformBuffer>,
    pub circle_collider_buff: HashMap<Entity, CircleColliderBuffer>,
    pub polygon_collider_buff: HashMap<Entity, ConvexPolygonBuffer>,
    pub rectangle_collider_buff: HashMap<Entity, RectangleColliderBuffer>
}

impl Default for ComponentTextBuffers {
    fn default() -> Self {
        Self {
            transform_buff: HashMap::new(),
            circle_collider_buff: HashMap::new(),
            polygon_collider_buff: HashMap::new(),
            rectangle_collider_buff: HashMap::new(),
        }
    }
}
pub struct TransformBuffer{
    pub pos_x: String,
    pub pos_y: String,
    pub rot_x: String,
    pub rot_y: String,
    pub scale_factor: String
}

pub struct  CircleColliderBuffer {
    pub radius: String
}

pub struct ConvexPolygonBuffer {
    pub circum_radius: String,
    pub sides: String
}

pub struct RectangleColliderBuffer {
    pub width: String,
    pub height: String
}