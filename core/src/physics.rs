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

#[derive(Bundle)]
pub struct PhysicsBundle {
    pub position: Position,
    pub rotation: Rotation,
    pub momentum: Momentum,
    pub angular_momentum: AngularMomentum,
    pub force: Force,
    pub torque: Torque,
    pub total_mass_position: TotalMassPosition,
    pub center_of_mass: CenterOfMass,
    pub mass: Mass,
    pub inertia: Inertia,
    pub inv_mass: InvMass,
    pub inv_inertia: InvInertia,
}

impl PhysicsBundle {
    fn new(
        position: FVec,
        rotation: Rot,
        velocity: FVec,
        angular_velocity: Rot,
        masses_iter: impl Iterator<Item = (IVec, i64)>,
    ) -> Self {
        let mut total_mass = 0;
        let mut total_mass_position = LVec::zero();
        let mut total_inertia = ULMat::zero();
        for (pos, mass) in masses_iter {
            total_mass_position += LVec::from(pos) * mass;
            total_mass += mass;
        }
        let inv_mass = 1.0 / (total_mass as f32);
        let inertia_mat = total_inertia.as_f32();
        let inv_inertia = inertia_mat.inversed();
        PhysicsBundle {
            position: Position(position),
            rotation: Rotation(rotation),
            momentum: Momentum((total_mass as f32) * velocity),
            angular_momentum: AngularMomentum(/*inertia_mat * angular_velocity*/todo!()),
            force: Force(FVec::zero()),
            torque: Torque(FVec::zero()),
            total_mass_position: TotalMassPosition(total_mass_position),
            center_of_mass: CenterOfMass(total_mass_position.as_f32() / (total_mass as f32)),
            mass: Mass(total_mass),
            inertia: Inertia(total_inertia),
            inv_mass: InvMass(inv_mass),
            inv_inertia: InvInertia(inv_inertia),
        }
    }
}

pub struct Timestep(pub f32);

pub struct Position(pub FVec);
pub struct Rotation(pub Rot);

pub struct Momentum(pub FVec);
pub struct AngularMomentum(pub FVec);

pub struct Force(pub FVec);
pub struct Torque(pub FVec);

// The sum of the positions of all the masses within the object.
pub struct TotalMassPosition(pub LVec);
// The center of mass relative to the object.
pub struct CenterOfMass(pub FVec);

// i64 in case of exotic matter types.
pub struct Mass(pub i64);
// The inertia of an object with respect to the object's space.
pub struct Inertia(pub ULMat);
pub struct InvMass(pub f32);
pub struct InvInertia(pub FMat);

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
