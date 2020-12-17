/// A voxel storage.
/// Type parameters:
/// Generally, the voxel storage constructor should take in a "default" voxel,
/// which is used to cull to a reasonable size.
///   - T: The type of the Voxel
pub trait VoxelStorage<T: Eq + Copy> {
    type Position: Copy;
    type Mutator<'a>: Writer<T>;
    type PositionIterator: Iterator<Item = Self::Position>;
    /// Gets the voxel at a position.
    fn get(&self, position: Self::Position) -> T;
    /// Gets the voxel at a position, with mutation.
    fn get_mut(&mut self, position: Self::Position) -> Self::Mutator<'_>;
    /// An iterator over all voxel positions.
    fn for_each(&self, f: impl FnMut(Self::Position, T));
    /// Whether a position is contained within the storage.
    /// If this is true, then the get_mut Writer set method should be O(1),
    /// and should not allocate any memory
    fn contains(&self, a: Self::Position) -> bool;
}

pub trait Writer<T> {
    /// Accesses the value pointed at by the writer.
    fn get(&mut self) -> T;
    /// Gets a mutable pointer to the value pointed at by the writer.
    fn get_mut(&mut self) -> &mut T;
}

/// A voxel storage which allows indexing of voxels.
/// The voxel index must be unique across all storages of the same type.
pub trait IndexableVoxelStorage<T: Eq + Copy>: VoxelStorage<T> {
    type Index = u64;
    /// Computes the index.
    fn index(&self, position: Self::Position) -> Self::Index;
    /// Computes the index and the value of the voxel.
    /// This purely exists as it may be more efficient than computing the index
    /// separately in some cases.
    fn index_get(&self, position: Self::Position) -> (Self::Index, T) {
        (self.index(position), self.get(position))
    }
    fn index_get_mut(&mut self, position: Self::Position) -> (Self::Index, Self::Mutator<'_>) {
        (self.index(position), self.get_mut(position))
    }
}

/// A voxel storage which supports collisions.
pub trait CollidableVoxelGrid<T: Eq + Copy>: VoxelStorage<T> {
    /// An arbitrary type that allows for easy collision detection
    /// between two voxel grids of the same type.
    type Collider;
    /// Gets the collider for the storage.
    fn collider(&self) -> Self::Collider;
}

pub mod chunk_map;
