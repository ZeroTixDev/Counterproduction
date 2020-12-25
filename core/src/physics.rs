use crate::geometry::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::tasks::{ComputeTaskPool, ParallelIterator};

pub struct PhysicsPlugin {
    pub timestep: f64,
}
impl Default for PhysicsPlugin {
    fn default() -> Self {
        PhysicsPlugin {
            timestep: 1.0 / 60.0,
        }
    }
}
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(Timestep(self.timestep as f32))
            .add_stage_after(
                stage::UPDATE,
                "physics",
                SystemStage::parallel()
                    .with_run_criteria(FixedTimestep::step(self.timestep))
                    // .with_system(linear_update.system()),
                    // .with_system(angular_update.system()),
            );
    }
}

pub struct PhysicsBundle {
    position: Position,
    rotation: Rotation,
    momentum: AngularMomentum,
    angular_momentum: AngularMomentum,
    force: Force,
    torque: Torque,
    total_mass_position: TotalMassPosition,
    center_of_mass: CenterOfMass,
    mass: Mass,
    inertia: Inertia,
    inv_mass: InvMass,
    inv_inertia: InvInertia,
}

pub struct Timestep(pub f32);

pub struct Position(pub FVec);
pub struct Rotation(pub Rot);

pub struct Momentum(pub FVec);
pub struct AngularMomentum(pub FVec);

pub struct Force(pub FVec);
pub struct Torque(pub FVec);

// The sum of the positions of all the masses within the object.
pub struct TotalMassPosition(pub [i64; 3]);
// The center of mass relative to the object.
pub struct CenterOfMass(pub FVec);

// i64 in case of exotic matter types.
pub struct Mass(pub i64);
pub struct Inertia(pub [[i64; 3]; 3]);
pub struct InvMass(pub i64);
pub struct InvInertia(pub Mat);
/*
fn linear_update(
    timestep: Res<Timestep>,
    pool: Res<ComputeTaskPool>,
    mut query: Query<(&mut Position, &mut Velocity, &mut Force, &InvMass)>,
) {
    query
        .par_iter_mut(32)
        .for_each(&pool.0, |(mut s, mut v, mut f, im)| {
            v.0 += f.0 * im.0 * timestep.0;
            s.0 += v.0 * timestep.0;
            f.0 = FVec::zero();
        })
}
*/
/*
fn angular_update(
    timestep: Res<Timestep>,
    pool: Res<ComputeTaskPool>,
    mut query: Query<(&mut Position, &mut Velocity, &mut Torque, &Inertia)>,
) {
    query
        .par_iter_mut(32)
        .for_each(&pool.0, |(mut s, mut v, mut t, i)| {
            v.0 += a.0 * timestep.0;
            s.0 += v.0 * timestep.0;
        })
}
*/
