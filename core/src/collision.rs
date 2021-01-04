use crate::geometry::FVec;
use crate::geometry::Rot;
use derive_new::*;

pub mod cube;
pub mod octree;

#[derive(new, Copy, Clone, PartialEq, Debug)]
pub struct Positioned<T> {
    pub object: T,
    pub position: FVec,
    pub rotation: Rot,
    /* Insert when necessary.
    pub velocity: FVec,
    pub angular_velocity: Rot,
    */
}

/// A collision result.
#[derive(new, Copy, Clone, PartialEq, Debug)]
pub struct CollisionResult {
    /// The penetration of the collision.
    pub penetration: FVec,
    pub collided: bool,
}

pub type VoxelCollisionList<P> = Vec<(P, P, FVec)>;

pub trait CollisionResolver {
    type Collider;
    type Position;
    fn collide(
        a: Positioned<Self::Collider>,
        b: Positioned<Self::Collider>,
    ) -> VoxelCollisionList<Self::Position>;
}
