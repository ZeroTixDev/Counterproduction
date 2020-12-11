struct Positioned<T> {
    t: T,
    position: FVec,
    rotation: Rot,
    velocity: FVec,
    angular_velocity: Rot,
}

trait VoxelCollisionResolver {
    type Collider;
    fn collide<X: CollidableVoxelGrid<Collider = Self::Collider>>(collidables: impl Iterator<Item = Positioned<X>>) -> VoxelCollisionList;
}
