use crate::for_each::ForEachMut;
use crate::geometry::*;
use bevy::prelude::*;
use bevy::tasks::{ComputeTaskPool, ParallelIterator};
use std::ops::Add;
use std::ops::Mul;
use std::ops::Neg;
use ultraviolet::Bivec3;

pub struct PhysicsPlugin {
    pub timestep: f64,
    // This is the schedule that the physics is added to.
    pub physics_schedule_name: Option<&'static str>,
}
impl Default for PhysicsPlugin {
    fn default() -> Self {
        PhysicsPlugin {
            timestep: 1.0 / 60.0,
            physics_schedule_name: None,
        }
    }
}
impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        let schedule_name = self.physics_schedule_name.unwrap_or_else(|| {
            let name = "physics-schedule";
            app.add_stage_after(
                stage::UPDATE,
                name,
                Schedule::default().with_stage("collide", SystemStage::serial()),
            );
            name
        });
        app.add_resource(Timestep(self.timestep as f32)).stage(
            schedule_name,
            |schedule: &mut Schedule| {
                schedule
                    .add_stage_before(
                        "collide",
                        "physics",
                        SystemStage::parallel()
                            .with_system(linear_update.system())
                            .with_system(angular_update.system()),
                    )
                    .add_stage_before(
                        "physics",
                        "pre-physics",
                        SystemStage::serial()
                            .with_system(recompute_after_changed_body.system())
                            .with_system(recompute_computed_after_changed.system()),
                    )
            },
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
            total_inertia += voxel_inertia(pos, mass);
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
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Timestep(pub f32);

/// The position of the object's center of mass.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Position(pub FVec);
/// The rotation of the object around its center of mass.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Rotation(pub Rot);

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Momentum(pub FVec);
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct AngularMomentum(pub FVec);

#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Force(pub FVec);
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Torque(pub FVec);

/// The sum of the positions of all the masses within the object.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct TotalMassPosition(pub LVec);
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Mass(pub i64);
/// The inertia of an object with respect to the object's origin.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct Inertia(pub LMat);
// == Computed properties == //
/// The center of mass relative to the object's origin.
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct CenterOfMass(pub FVec);
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct InvMass(pub f32);
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct InertiaAroundCenterOfMass(pub FMat);
#[derive(Copy, Clone, PartialEq, Default, Debug)]
pub struct InvInertiaAroundCenterOfMass(pub FMat);

/// A struct representing a change in mass of an object.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct ChangedBodies(pub Vec<(IVec, i64)>);

fn inertia_of_position<T: Add<Output = T> + Mul<Output = T> + Neg<Output = T> + Copy>(
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

// This function is inaccurate; the result should be divided by 24,
// but generally it doesn't matter.
fn voxel_inertia(pos: IVec, mass: i64) -> LMat {
    LMat::identity() * mass + inertia_of_position(LVec::from(pos).into(), mass).into()
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
                i.0 += inertia_of_position(pos.into(), mass).into();
            }
        })
}

// TODO: AT THE START, com.0 IS ZERO. MAKE IT SO THAT THE POSITION IS NOT
// CHANGED DUE TO THIS.
#[allow(clippy::type_complexity)]
fn recompute_computed_after_changed(
    pool: Res<ComputeTaskPool>,
    mut query: Query<
        (
            &TotalMassPosition,
            &Mass,
            &Inertia,
            &Rotation,
            &mut Position,
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
        |(tmp, m, i, r, mut p, mut com, mut im, mut iacom, mut iiacom)| {
            let old_com = com.0;
            com.0 = tmp.0.as_f32() / (m.0 as f32);
            // TODO: Check these parts.
            let del_com = com.0 - old_com;
            let rot_del_com = r.0.reversed() * del_com;
            p.0 += rot_del_com;
            im.0 = 1.0 / (m.0 as f32);
            iacom.0 = i.0.as_f32() + inertia_of_position(*com.0.as_array(), m.0 as f32).into();
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
            debug_assert!(!f32::is_nan(p.0.x) && !f32::is_nan(p.0.y) && !f32::is_nan(p.0.z));
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
            if w == FVec::zero() {
                return;
            }
            let theta = w.mag();
            let half = theta / 2.0;
            let vec = w.normalized() * half.sin();
            // TODO: TEST THIS EQUATION AND MAKE SURE IT WORKS
            r.0 = Rot::new(half.cos(), Bivec3::new(vec.z, vec.y, vec.x)) * r.0;
            t.0 = FVec::zero();
            debug_assert!(!f32::is_nan(r.0.s));
        });
}

pub fn apply_force(force: FVec, position: FVec, ftp: &mut (Mut<Force>, Mut<Torque>, &Position)) {
    let delta = position - ftp.2 .0;
    ftp.0 .0 += force;
    ftp.1 .0 += force.cross(delta);
}

// Inertia computations taken from http://www.kwon3d.com/theory/moi/triten.html
// Other physics from both         http://www.cs.cmu.edu/~baraff/sigcourse/notesd1.pdf
// and                             http://developer.nvidia.com/gpugems/gpugems3/part-v-physics-simulation/chapter-29-real-time-rigid-body-simulation-gpus

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(PartialEq, Copy, Clone, Default, Debug)]
    struct CubeForce(FVec);
    // Relative to the center of mass of the cube, but not rotated.
    #[derive(PartialEq, Copy, Clone, Default, Debug)]
    struct CubeForcePos(FVec);

    fn init_app(timestep: f64) -> AppBuilder {
        let mut app = App::build();
        app.add_plugin(bevy::reflect::ReflectPlugin)
            .add_plugin(bevy::core::CorePlugin)
            .add_plugin(PhysicsPlugin {
                timestep,
                physics_schedule_name: None,
            });
        app
    }

    fn apply_force_simple(
        cube_force: Res<CubeForce>,
        cube_force_pos: Res<CubeForcePos>,
        mut query: Query<(&mut Force, &mut Torque, &Position)>,
    ) {
        for mut q in query.iter_mut() {
            apply_force(cube_force.0, q.2 .0 + cube_force_pos.0, &mut q);
        }
    }

    fn init_cube(commands: &mut Commands) {
        commands.spawn(PhysicsBundle::new(
            FVec::zero(),
            Rot::identity(),
            FVec::zero(),
            vec![(IVec::zero(), 1)],
        ));
    }

    fn assert_close(a: FVec, b: FVec) {
        if (a - b).mag_sq() > 0.001 {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_simple_movement() {
        let mut app = init_app(1.0);
        app.add_startup_system(init_cube.system())
            .add_system(apply_force_simple.system())
            .add_resource(CubeForce(FVec::new(1.0, 0.0, 0.0)))
            .add_resource(CubeForcePos::default());
        let mut app = app.app;
        app.update();
        app.update();
        for (pos, momentum) in app.world.query::<(&Position, &Momentum)>() {
            assert_close(pos.0, FVec::new(2.0, 0.0, 0.0));
            assert_close(momentum.0, FVec::new(2.0, 0.0, 0.0));
        }
        app.resources.get_mut::<CubeForce>().unwrap().0 = FVec::new(-1.0, 0.0, 0.0);
        app.update();
        app.update();
        for (pos, momentum) in app.world.query::<(&Position, &Momentum)>() {
            assert_close(pos.0, FVec::new(3.0, 0.0, 0.0));
            assert_close(momentum.0, FVec::new(0.0, 0.0, 0.0));
        }
    }

    #[test]
    fn test_rotation() {
        let mut app = init_app(1.0);
        app.add_startup_system(init_cube.system())
            .add_system(apply_force_simple.system())
            .add_resource(CubeForce(FVec::new(1.0, 0.0, 0.0)))
            .add_resource(CubeForcePos(FVec::new(0.0, 1.0, 0.0)));
        let mut app = app.app;
        app.update();
        app.update();
        for rot in app.world.query::<&Rotation>() {
            println!("{:?}", rot);
        }
        panic!();
    }
}
