use crate::geometry::*;
use crate::storage::Octree;
use crate::storage::*;
use crate::voxel::Voxel;
use derive_new::*;

#[derive(new)]
pub struct Entity<Grid: IndexableVoxelStorage<Voxel>> {
    pub transform: Isometry,
    pub grid: Grid,
    pub tree: Octree,
}
