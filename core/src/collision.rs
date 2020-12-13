use crate::geometry::FVec;
use crate::geometry::Rot;
use crate::storage::CollidableVoxelGrid;
use derive_new::*;
use fnv::FnvHashMap;

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
    pub penetration: f32,
    pub collided: bool,
}

pub type VoxelCollisionListInterior<P> = FnvHashMap<(usize, usize), Vec<(P, P, CollisionResult)>>;
#[derive(new, Clone, PartialEq, Debug)]
pub struct VoxelCollisionList<P> {
    /// The map of all collisions.
    /// The first index in the `(usize, usize)`
    /// should always be the smaller one.
    pub data: VoxelCollisionListInterior<P>,
}

trait CollisionResolver {
    type Collider;
    type Position;
    fn collide<
        T: Eq + Copy,
        X: CollidableVoxelGrid<T, Collider = Self::Collider, Position = Self::Position>,
    >(
        collidables: impl Iterator<Item = Positioned<Self::Collider>>,
    ) -> VoxelCollisionList<Self::Position>;
}
