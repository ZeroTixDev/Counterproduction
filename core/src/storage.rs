/// A voxel storage.
/// Type parameters:
///   - T: The type of the Voxel
/// Generally, the voxel storage constructor should take in a "default" voxel,
/// which is used to cull to a reasonable size.
trait VoxelStorage<T: Eq> {
    type Position: Copy;
    type Shared: Read<T>;
    type Mutable: Read<T> + Write<T>;
    type PositionIterator: Iterator<Item = Self::Position>;
    /// Gets the voxel at a position.
    fn get(&self, position: Self::Position) -> Self::Shared;
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
    fn partition<F: Fn(Self::Position, Self::Shared) -> bool>(&mut self, test: F) -> Self;
}

trait Read<T> {
    fn get(&self) -> &T;
}
trait Write<T> {
    fn set(&mut self, value: T) -> &mut T;
}

/// A voxel storage which allows indexing of voxels.
/// The voxel index must be unique for the storage.
trait IndexableVoxelStorage<T: Eq>: VoxelStorage<T> {
    type Index = u64;
    fn index(&self, position: Self::Position) -> Self::Index;
    fn index_get(&self, position: Self::Position) -> (Self::Index, Self::Shared) {
        (self.index(position), self.get(position))
    }
    fn index_get_mut(&mut self, position: Self::Position) -> (Self::Index, Self::Mutable) {
        (self.index(position), self.get_mut(position))
    }
}
