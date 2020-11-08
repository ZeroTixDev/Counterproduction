#![allow(incomplete_features)]
#![feature(arbitrary_self_types)]
#![feature(generic_associated_types)]
#![feature(const_generics)]
#![feature(iterator_fold_self)]
#![feature(clamp)]
#![allow(clippy::type_complexity)]
// What's a test? Never heard of such a thing.

pub mod entities;
use entities::*;
pub mod players;
use players::*;
pub mod ai;
use ai::*;
pub mod camera;
use camera::*;

use bevy::prelude::*;

pub fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EntityPlugin)
        .add_plugin(AIPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_startup_system(setup.system())
        .add_system_to_stage(stage::FIRST, step.system())
        .run();
}

fn step() {
    println!("=============================================================");
}

fn setup(mut commands: Commands) {
    commands
        .spawn(PlayerProps::new(
            Vec3::new(100.0, 0.0, 0.0),
            5.0,
            Color::rgb_u8(50, 168, 82),
        ))
        .with_children(|parent| {
            parent
                .spawn((PlayerUnit(Stats::new(6.0, 2.9, 30.0, 1.5, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(6.0, 2.9, 30.0, 1.5, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(6.0, 2.9, 30.0, 1.5, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(6.0, 2.9, 30.0, 1.5, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(6.0, 2.9, 30.0, 1.5, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(6.0, 2.9, 30.0, 1.5, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(6.0, 2.9, 30.0, 1.5, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(6.0, 2.9, 30.0, 1.5, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(6.0, 2.9, 30.0, 1.5, 1.0), AI::Simple),));
        })
        .spawn(PlayerProps::new(
            Vec3::new(-100.0, 0.0, 0.0),
            5.0,
            Color::rgb_u8(66, 135, 245),
        ))
        .with_children(|parent| {
        })
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 100.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 50.0, 200.0))
                .looking_at(Vec3::zero(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(CameraLook::new(
            std::f32::consts::PI,
            0.5,
            230.0,
            Vec3::zero(),
        ));
}
