use super::*;
use crate::geometry::IVec;
use building_blocks::prelude::*;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

static LAST_CHUNK_INDEX: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone)]
struct ChunkIndex(usize);

fn convert_to_point(a: IVec) -> PointN<[i32; 3]> {
    PointN(a.as_array())
}

pub struct ChunkStorage<T: 'static + Eq + Copy> {
    map: ChunkHashMap3<T, ChunkIndex>,
}

pub struct Mutator<'a, T: 'static + Eq + Copy> {
    storage: &'a mut ChunkStorage<T>,
    position: IVec,
}

impl<'a, T: 'static + Eq + Copy> Deref for Mutator<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.storage.get(self.position)
    }
}

impl<'a, T: 'static + Eq + Copy> DerefMut for Mutator<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        let ambient = self.storage.map.ambient_value();
        let create_chunk = |_: PointN<[i32; 3]>, e: ExtentN<[i32; 3]>| -> Chunk3<T, ChunkIndex> {
            let current_number = ChunkIndex(LAST_CHUNK_INDEX.fetch_add(1, Ordering::Relaxed));
            Chunk3 {
                metadata: current_number,
                array: Array3::fill(e, ambient),
            }
        };

        let (_, value) = self
            .storage
            .map
            .get_mut_point_or_insert_chunk_with(&convert_to_point(self.position), create_chunk);
        value
    }
}

impl<'a, T: 'static + Eq + Copy> Writer<T> for Mutator<'a, T> {}

pub struct PositionIterator;

impl Iterator for PositionIterator {
    type Item = IVec;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

impl<T: 'static + Eq + Copy> VoxelStorage<T> for ChunkStorage<T> {
    type Position = IVec;
    type Mutator<'a> = Mutator<'a, T>;
    type PositionIterator = PositionIterator;

    fn get(&self, position: Self::Position) -> &T {
        todo!()
    }

    /// Finish this thing and make it create the thing and increment
    /// LAST_CHUNK_INDEX.
    fn get_mut<'a>(&'a mut self, position: Self::Position) -> Self::Mutator<'a> {
        Mutator {
            storage: self,
            position,
        }
    }
    fn all(&self) -> Self::PositionIterator {
        todo!()
    }
    fn partition<F: Fn(Self::Position, &T) -> bool>(&mut self, test: F) -> Self {
        todo!()
    }
    fn contains(&self, a: Self::Position) -> bool {
        todo!()
    }
}
