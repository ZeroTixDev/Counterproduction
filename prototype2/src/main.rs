#![allow(incomplete_features)]
#![feature(arbitrary_self_types)]
#![feature(generic_associated_types)]
#![feature(const_generics)]
// What's a test? Never heard of such a thing.

pub mod entities;

use entities::*;
use shape::Cube;

use bevy::prelude::*;

pub fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_startup_system(setup.system())
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let body_color = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
    let gun_color = materials.add(Color::rgb(0.6, 0.7, 0.5).into());
    // add entities to the world
    commands
        // plane
        // cube
        .spawn(Unit::new(
            Default::default(),
            Stats::new(5.0, 1.0, 5.0, 1.0),
        ))
        .with_bundle(PbrComponents {
            mesh: meshes.add(Mesh::from(Cube { size: 1.0 })),
            material: body_color,
            transform: Transform::from_translation(Default::default()),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn(PbrComponents {
                mesh: meshes.add(Mesh::from(Cube { size: 0.5 })),
                material: gun_color.clone(),
                transform: Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
                ..Default::default()
            });
        })
        // light
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}
