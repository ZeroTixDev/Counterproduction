#![allow(incomplete_features)]
#![feature(arbitrary_self_types)]
#![feature(generic_associated_types)]
#![feature(const_generics)]
#![feature(iterator_fold_self)]
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
        .add_plugins(DefaultPlugins)
        .add_plugin(EntityPlugin)
        .add_plugin(AIPlugin)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup.system())
        .run();
}

fn setup(mut commands: Commands) {
    commands
        .spawn(PlayerProps::new(
            Vec3::new(100.0, 0.0, 0.0),
            5.0,
            Color::rgb_u8(50, 168, 82),
        ))
        .with_children(|parent| {
            parent.spawn((PlayerUnit(Default::default(), AI::Simple),));
        })
        .spawn(PlayerProps::new(
            Vec3::new(-100.0, 5.0, 0.0),
            5.0,
            Color::rgb_u8(66, 135, 245),
        ))
        .with_children(|parent| {
            parent.spawn((PlayerUnit(Default::default(), AI::Simple),));
        })
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(12.0, 24.0, 12.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 200.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        });
}
