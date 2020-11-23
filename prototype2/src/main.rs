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
pub mod objective;
use objective::*;

use bevy::prelude::*;
use bevy_mod_picking::*;

struct CurrentPlayer(Entity);

pub fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(EntityPlugin)
        .add_plugin(AIPlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(CameraPlugin)
        .add_plugin(PickingPlugin)
        .add_startup_system(setup.system())
        .add_system(set_objective.system())
        .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let current_player = commands
        .spawn(PlayerProps::new(
            Vec3::new(80.0, 0.0, 0.0),
            5.0,
            Color::rgb_u8(50, 168, 82),
        ))
        .with_children(|parent| {
            parent
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(1.0, 1.0, 5.0, 3.0, 1.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(9.0, 3.0, 30.0, 0.5, 5.0), AI::Simple),));
        })
        .current_entity()
        .unwrap();
    commands
        .spawn(PlayerProps::new(
            Vec3::new(-80.0, 0.0, 0.0),
            5.0,
            Color::rgb_u8(66, 135, 245),
        ))
        .with_children(|parent| {
            parent
                .spawn((PlayerUnit(Stats::new(9.0, 3.0, 30.0, 1.0, 3.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(9.0, 3.0, 30.0, 1.0, 3.0), AI::Simple),))
                .spawn((PlayerUnit(Stats::new(9.0, 3.0, 30.0, 1.0, 3.0), AI::Simple),));
        });
    commands.insert_resource(CurrentPlayer(current_player));
    commands
        .spawn((
            meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
            Transform::default(),
            GlobalTransform::default(),
            Draw::default(),
        ))
        .with(PickableMesh::default())
        .spawn((
            meshes.add(Mesh::from(shape::Plane { size: 1000.0 })),
            Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::PI)),
            GlobalTransform::default(),
            Draw::default(),
        ))
        .with(PickableMesh::default())
        .spawn(LightComponents {
            transform: Transform::from_translation(Vec3::new(0.0, 200.0, 0.0)),
            ..Default::default()
        })
        .spawn(Camera3dComponents {
            transform: Transform::from_translation(Vec3::new(0.0, -25.0, 150.0))
                .looking_at(Vec3::zero(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(CameraLook::new(
            std::f32::consts::PI,
            0.5,
            150.0,
            Vec3::new(0.0, -25.0, 0.0),
        ))
        .with(PickSource::default());
}
fn set_objective(
    mut commands: Commands,
    pick_state: Res<PickState>,
    mouse_button_input: Res<Input<MouseButton>>,
    player: Res<CurrentPlayer>,
) {
    if let Some(pick) = pick_state.top(Group::default()) {
        let location = pick.1.position();
        if mouse_button_input.just_pressed(MouseButton::Left) {
            commands
                .remove_one::<Objective>(player.0)
                .insert_one(player.0, Objective::new(*location));
        }
    }
}
