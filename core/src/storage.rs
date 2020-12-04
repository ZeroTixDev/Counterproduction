/// A voxel storage.
/// Type parameters:
/// Generally, the voxel storage constructor should take in a "default" voxel,
/// which is used to cull to a reasonable size.
///   - T: The type of the Voxel
pub trait VoxelStorage<T: Eq> {
    type Position: Copy;
    type Mutable: Writer<T>;
    type PositionIterator: Iterator<Item = Self::Position>;
    /// Gets the voxel at a position.
    fn get(&self, position: Self::Position) -> &T;
    /// Gets the voxel at a position, with mutation.
    fn get_mut(&mut self, position: Self::Position) -> Self::Mutable;
    /// An iterator over all voxel positions.
    fn all(&self) -> Self::PositionIterator;
    /// Splits the world into two separate storages. The current storage
    /// includes all voxels for which the function `test` returns `true`,
    /// while the return value includes all voxels for which the function
    /// `test` returns `false`.
    ///
    /// If this voxel storage is an IndexableVoxelStorage, `partition` must not
    /// change what `index` any of the voxels have.
    fn partition<F: Fn(Self::Position, &T) -> bool>(&mut self, test: F) -> Self;
    /// Whether a position is contained within the storage.
    /// If this is true, then the get_mut Writer set method should be O(1),
    /// and should not allocate any memory
    fn contains(&self, a: Self::Position) -> bool;
}

pub trait Writer<T> {
    fn get(&self) -> &T;
    fn set(&mut self, value: T) -> &mut T;
}

/// A voxel storage which allows indexing of voxels.
/// The voxel index must be unique across all storages.
pub trait IndexableVoxelStorage<T: Eq>: VoxelStorage<T> {
    type Index = u64;
    /// Computes the index.
    fn index(&self, position: Self::Position) -> Self::Index;
    /// Computes the index and the value of the voxel.
    /// This purely exists as it may be more efficient than computing the index
    /// separately in some cases.
    fn index_get(&self, position: Self::Position) -> (Self::Index, &T) {
        (self.index(position), self.get(position))
    }
    fn index_get_mut(&mut self, position: Self::Position) -> (Self::Index, Self::Mutable) {
        (self.index(position), self.get_mut(position))
    }
}

pub type Octree = building_blocks::partition::Octree;
