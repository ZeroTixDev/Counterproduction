#![allow(incomplete_features)]
#![feature(arbitrary_self_types)]
#![feature(generic_associated_types)]
#![feature(const_generics)]
// What's a test? Never heard of such a thing.

pub mod entities;

use entities::*;

use bevy::prelude::*;

pub fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_default_plugins()
        .add_plugin(EntityPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(UnitProps::new(
            Default::default(),
            Stats::new(5.0, 1.0, 5.0, 1.0),
        ))
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(3.0, 5.0, 8.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}
