use crate::for_each::ForEach;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Index;

/// A voxel storage.
/// Type parameters:
/// Generally, the voxel storage constructor should take in a "default" voxel,
/// which is used to cull to a reasonable size.
pub trait VoxelStorage:
    Index<<Self as VoxelStorage>::Position, Output = <Self as VoxelStorage>::T>
    + ForEach<(<Self as VoxelStorage>::Position, <Self as VoxelStorage>::T)> {
    /// The voxel type.
    type T: Eq + Copy;
    type Position: Eq + Copy;
    type Mutator<'a>: Writer<Self::T>;
    type PositionIterator: Iterator<Item = Self::Position>;
    /// Gets the voxel at a position.
    fn get(&self, position: Self::Position) -> &Self::T;
    /// Gets the voxel at a position, with mutation.
    fn get_mut(&mut self, position: Self::Position) -> Self::Mutator<'_>;
    /// Whether a position is contained within the storage.
    /// If this is true, then the get_mut Writer set method should be O(1),
    /// and should not allocate any memory
    fn contains(&self, a: Self::Position) -> bool;
    /// Gets the ambient value of the storage.
    /// When the storage is initialized, calling `get` will always return the
    /// ambient value.
    fn ambient(&self) -> Self::T;
}

pub trait Writer<T> {
    /// Accesses the value pointed at by the writer.
    fn get(&mut self) -> &T;
    /// Gets a mutable pointer to the value pointed at by the writer.
    fn get_mut(&mut self) -> &mut T;
}

/// A voxel storage which allows indexing of voxels.
/// The voxel index must be unique across all storages of the same type.
pub trait IndexableVoxelStorage: VoxelStorage {
    type Index: Hash + Eq + Copy;
    /// Computes the index.
    fn index_of(&self, position: Self::Position) -> Option<Self::Index>;
    /// Computes the index and the value of the voxel.
    /// This purely exists as it may be more efficient than computing the index
    /// separately in some cases.
    fn index_get(&self, position: Self::Position) -> (Option<Self::Index>, &Self::T) {
        (self.index_of(position), self.get(position))
    }
    fn index_get_mut(
        &mut self,
        position: Self::Position,
    ) -> (Option<Self::Index>, Self::Mutator<'_>) {
        (self.index_of(position), self.get_mut(position))
    }
}

/// A voxel storage which supports collisions.
pub trait CollidableVoxelGrid: VoxelStorage {
    /// An arbitrary type that allows for easy collision detection
    /// between two voxel grids of the same type.
    type Collider;
    /// Gets the collider for the storage.
    fn collider(&self) -> Self::Collider;
}

macro_rules! impl_index {
    ($name: ident, $ty: ident) => {
        impl<$ty: Eq + Copy> Index<<Self as VoxelStorage>::Position> for $name<$ty> {
            type Output = $ty;
            fn index(&self, position: <Self as VoxelStorage>::Position) -> &Self::Output {
                self.get(position)
            }
        }
    };
}

pub fn test_storage<S: VoxelStorage>(storage: &mut S, value: S::T, pos: S::Position)
where
    S::T: Debug, {
    let ambient = storage.ambient();
    assert_eq!(storage[pos], ambient);
    {
        let mut mut_ref = storage.get_mut(pos);
        assert_eq!(*mut_ref.get(), ambient);
        *mut_ref.get_mut() = value;
        assert_eq!(*mut_ref.get(), value);
    }
    assert_eq!(*storage.get(pos), value);
    {
        let mut mut_ref = storage.get_mut(pos);
        assert_eq!(*mut_ref.get_mut(), value);
        *mut_ref.get_mut() = ambient;
        assert_eq!(*mut_ref.get(), ambient);
    }
    assert_eq!(*storage.get(pos), ambient);
}

pub mod chunk_map;
