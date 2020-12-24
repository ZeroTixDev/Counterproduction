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
                    .with_system(linear_update.system()), // .with_system(angular_update.system()),
            );
    }
}

pub struct Timestep(pub f32);

pub struct Position(pub FVec);
pub struct Rotation(pub Rot);

pub struct Velocity(pub FVec);
pub struct AngularVelocity(pub Rot);

pub struct Force(pub FVec);
pub struct Torque(pub FVec);

/* Figure out how to use */
// pub struct CenterOfMass(pub FVec);
pub struct Mass(pub f32);
// pub struct Inertia(pub Mat); // TODO: FIGURE OUT HOW TO USE THIS
pub struct InvMass(pub f32);

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
