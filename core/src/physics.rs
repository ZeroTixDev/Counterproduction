use crate::for_each::ForEachMut;
use crate::geometry::*;
use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy::tasks::{ComputeTaskPool, ParallelIterator};
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use ultraviolet::Bivec3;

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
                    .with_system(linear_update.system())
                    .with_system(angular_update.system()),
            )
            .add_stage_after(
                stage::UPDATE,
                "pre-physics",
                SystemStage::serial()
                    .with_system(recompute_after_changed_body.system())
                    .with_system(recompute_computed_after_changed.system()),
            );
    }
}

#[derive(Bundle)]
/// A bundle for physics.
/// IMPORTANT: DO NOT ADD THIS AFTER THE UPDATE STAGE!
/// ADDING AFTER THE UPDATE STAGE WILL CAUSE INVALID STATES.
pub struct PhysicsBundle {
    position: Position,
    rotation: Rotation,
    momentum: Momentum,
    angular_momentum: AngularMomentum,
    force: Force,
    torque: Torque,
    total_mass_position: TotalMassPosition,
    mass: Mass,
    inertia: Inertia,
    changed_bodies: ChangedBodies,
    // Computed fields
    center_of_mass: CenterOfMass,
    inv_mass: InvMass,
    inertia_around_center_of_mass: InertiaAroundCenterOfMass,
    inv_inertia_around_center_of_mass: InvInertiaAroundCenterOfMass,
}

impl PhysicsBundle {
    pub fn new(
        position: FVec,
        rotation: Rot,
        velocity: FVec,
        /* angular_velocity: FVec, */
        mut masses_fn: impl ForEachMut<(IVec, i64)>,
    ) -> Self {
        let mut total_mass = 0;
        let mut total_mass_position = LVec::zero();
        let mut total_inertia = LMat::zero();
        masses_fn.for_each_mut(|(pos, mass)| {
            total_mass_position += LVec::from(pos) * mass;
            total_mass += mass;
            total_inertia += inertia_of(LVec::from(pos).into(), mass).into();
        });
        PhysicsBundle {
            position: Position(position),
            rotation: Rotation(rotation),
            momentum: Momentum((total_mass as f32) * velocity),
            angular_momentum: AngularMomentum(FVec::zero()),
            force: Force(FVec::zero()),
            torque: Torque(FVec::zero()),
            total_mass_position: TotalMassPosition(total_mass_position),
            mass: Mass(total_mass),
            inertia: Inertia(total_inertia),
            changed_bodies: ChangedBodies(vec![]),
            // These are all computed in the pre-physics stage.
            // As such, all of them are default values.
            center_of_mass: CenterOfMass(FVec::zero()),
            inv_mass: InvMass(0.0),
            inertia_around_center_of_mass: InertiaAroundCenterOfMass(FMat::identity()),
            inv_inertia_around_center_of_mass: InvInertiaAroundCenterOfMass(FMat::identity()),
        }
    }
}
pub struct Timestep(pub f32);

// The position of the object's center of mass.
pub struct Position(pub FVec);
// The rotation of the object around its center of mass.
pub struct Rotation(pub Rot);

pub struct Momentum(pub FVec);
pub struct AngularMomentum(pub FVec);

pub struct Force(pub FVec);
pub struct Torque(pub FVec);

// The sum of the positions of all the masses within the object.
pub struct TotalMassPosition(pub LVec);
pub struct Mass(pub i64);
// The inertia of an object with respect to the object's origin.
pub struct Inertia(pub LMat);
// == Computed properties == //
// The center of mass relative to the object's origin.
pub struct CenterOfMass(pub FVec);
pub struct InvMass(pub f32);
pub struct InertiaAroundCenterOfMass(pub FMat);
pub struct InvInertiaAroundCenterOfMass(pub FMat);

// A struct representing a change in mass of an object.
pub struct ChangedBodies(pub Vec<(IVec, i64)>);

fn inertia_of<T: Add<Output = T> + Mul<Output = T> + Neg<Output = T> + Copy>(
    pos: [T; 3],
    mass: T,
) -> [[T; 3]; 3] {
    [
        [
            mass * (pos[1] * pos[1] + pos[2] * pos[2]),
            -mass * pos[0] * pos[1],
            -mass * pos[0] * pos[2],
        ],
        [
            -mass * pos[1] * pos[0],
            mass * (pos[0] * pos[0] + pos[2] * pos[2]),
            -mass * pos[1] * pos[2],
        ],
        [
            -mass * pos[2] * pos[0],
            -mass * pos[2] * pos[1],
            mass * (pos[0] * pos[0] + pos[1] * pos[1]),
        ],
    ]
}

fn recompute_after_changed_body(
    pool: Res<ComputeTaskPool>,
    mut query: Query<
        (
            &mut ChangedBodies,
            &mut TotalMassPosition,
            &mut Mass,
            &mut Inertia,
        ),
        Mutated<ChangedBodies>,
    >,
) {
    query
        .par_iter_mut(64)
        .for_each(&pool.0, |(mut cb, mut tmp, mut m, mut i)| {
            for (pos, mass) in std::mem::replace(&mut cb.0, vec![]).into_iter() {
                let pos = LVec::from(pos);
                m.0 += mass;
                tmp.0 += pos * mass;
                i.0 += inertia_of(pos.into(), mass).into();
            }
        })
}

#[allow(clippy::type_complexity)]
fn recompute_computed_after_changed(
    pool: Res<ComputeTaskPool>,
    mut query: Query<
        (
            &TotalMassPosition,
            &Mass,
            &Inertia,
            &mut CenterOfMass,
            &mut InvMass,
            &mut InertiaAroundCenterOfMass,
            &mut InvInertiaAroundCenterOfMass,
        ),
        Or<(Changed<TotalMassPosition>, Changed<Mass>, Changed<Inertia>)>,
    >,
) {
    query.par_iter_mut(128).for_each(
        &pool.0,
        |(tmp, m, i, mut com, mut im, mut iacom, mut iiacom)| {
            com.0 = tmp.0.as_f32() / (m.0 as f32);
            im.0 = 1.0 / (m.0 as f32);
            iacom.0 = i.0.as_f32() + inertia_of(*com.0.as_array(), m.0 as f32).into();
            iiacom.0 = iacom.0.inversed();
        },
    );
}

fn linear_update(
    timestep: Res<Timestep>,
    pool: Res<ComputeTaskPool>,
    mut query: Query<(&InvMass, &mut Force, &mut Momentum, &mut Position)>,
) {
    let timestep = timestep.0;
    query
        .par_iter_mut(128)
        .for_each(&pool.0, |(im, mut f, mut m, mut p)| {
            m.0 += timestep * f.0;
            p.0 += timestep * m.0 * im.0;
            f.0 = FVec::zero();
        });
}

fn angular_update(
    timestep: Res<Timestep>,
    pool: Res<ComputeTaskPool>,
    mut query: Query<(
        &InvInertiaAroundCenterOfMass,
        &mut Torque,
        &mut AngularMomentum,
        &mut Rotation,
    )>,
) {
    let timestep = timestep.0;
    query
        .par_iter_mut(128)
        .for_each(&pool.0, |(iiacom, mut t, mut am, mut r)| {
            let rot_mat = r.0.into_matrix();
            am.0 += timestep * t.0;
            let w = rot_mat * iiacom.0 * rot_mat.inversed() * am.0;
            let theta = w.mag();
            let half = theta / 2.0;
            let vec = w.normalized() * half.sin();
            // TODO: TEST THIS EQUATION AND MAKE SURE IT WORKS
            r.0 += Rot::new(half.cos(), Bivec3::new(vec.z, vec.y, vec.x));
            t.0 = FVec::zero();
        });
}

// Inertia computations taken from http://www.kwon3d.com/theory/moi/triten.html
// Other physics from both         http://www.cs.cmu.edu/~baraff/sigcourse/notesd1.pdf
// and                             http://developer.nvidia.com/gpugems/gpugems3/part-v-physics-simulation/chapter-29-real-time-rigid-body-simulation-gpus
