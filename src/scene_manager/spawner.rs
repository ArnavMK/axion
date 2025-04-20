use bevy::prelude::*;
use crate::ui::events::*;

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
                    Mesh2d(meshes.add(Circle::new(50.))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.2, 0.1, 0.0)))),
                    Transform::from_translation(Vec3::new(-150., 0., 0.)),
                ));
            }
            CreateEntity::Triangle => {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(50., 100.))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.4, 0.3)))),
                    Transform::from_translation(Vec3::new(-50., 0., 0.)),
                ));
            }
            CreateEntity::Rectangle => {
                commands.spawn((
                    Mesh2d(meshes.add(Rectangle::new(50., 100.))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.5, 0.4, 0.3)))),
                    Transform::from_translation(Vec3::new(50., 0., 0.)),
                ));
            }
            CreateEntity::Hexagon => {                
                commands.spawn((
                    Mesh2d(meshes.add(RegularPolygon::new(50., 6))),
                    MeshMaterial2d(materials.add(ColorMaterial::from_color(Color::srgb(0.8, 0.7, 0.6)))),
                    Transform::from_translation(Vec3::new(150., 0., 0.)),
                ));
            }
        }

    }

}