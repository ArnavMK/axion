use bevy::prelude::*;
use crate::ui::events::*;
use crate::physics::collider::*;
use crate::physics::shapes::{circle::*, convex_polygon::*, rectangle::*};

pub fn handle_entity_spawning(
    mut spawner_events: EventReader<CreateEntity>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    
    for create in spawner_events.read() {

        match create {
            
            CreateEntity::Circle => {
                commands.spawn((
                    Mesh2d(meshes.add(Circle::new(50.0))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.1, 0.0)))),
                    Transform::from_translation(Vec3::new(0., 0., 0.)),
                    Collider {
                        shape: CircleShape {radius: 50.0}
                    }
                ));
            }

            CreateEntity::Rectangle => {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(50., 100.))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.4, 0.3)))),
                    Transform::from_translation(Vec3::new(0., 0., 0.)),
                    Collider {
                        shape: RectangleShape { width: 50.0, height: 100.0},
                    }
                ));
            }

            CreateEntity::ConvexPolygon => {                
                commands.spawn((
                    Mesh2d(meshes.add(RegularPolygon::new(50., 6))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.8, 0.7, 0.6)))),
                    Transform::from_translation(Vec3::new(0., 0., 0.)),
                    Collider {
                        shape: ConvexPolygonShape {circum_radius: 50.0, sides: 6}
                    }
                ));
            }
        }

    }

}