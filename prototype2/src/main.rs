#![allow(incomplete_features)]
#![feature(arbitrary_self_types)]
#![feature(generic_associated_types)]
#![feature(const_generics)]
#![allow(clippy::type_complexity)]
// What's a test? Never heard of such a thing.

pub mod entities;
use entities::*;
pub mod players;
use players::*;
pub mod ai;
use ai::*;

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
        .spawn(UnitProps::new(
            Vec3::new(5.0, 2.0, 1.0),
            Stats::new(5.0, 1.0, 5.0, 1.0),
        ))
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(12.0, 24.0, 12.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(12.0, 15.0, 24.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}
