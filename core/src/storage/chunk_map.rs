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
fn convert_from_point(a: PointN<[i32; 3]>) -> IVec {
    a.0.into()
}

pub struct ChunkStorage<T: 'static + Eq + Copy> {
    map: ChunkHashMap3<T, ChunkIndex>,
}

pub struct Mutator<'a, T: 'static + Eq + Copy> {
    storage: &'a mut ChunkStorage<T>,
    position: IVec,
}

impl<'a, T: 'static + Eq + Copy> Writer<T> for Mutator<'a, T> {
    fn get(&mut self) -> T {
        self.storage.get(self.position)
    }

    fn get_mut(&mut self) -> &mut T {
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

    fn get(&self, position: Self::Position) -> T {
        self.map.get(&convert_to_point(position))
    }
    fn get_mut(&mut self, position: Self::Position) -> Self::Mutator<'_> {
        Mutator {
            storage: self,
            position,
        }
    }
    fn for_each(&self, mut f: impl FnMut(Self::Position, T)) {
        let extent = self.map.bounding_extent();
        self.map
            .for_each(&extent, |p, t| f(convert_from_point(p), t));
    }
    fn contains(&self, position: Self::Position) -> bool {
        self.map
            .get_chunk_containing_point(&convert_to_point(position))
            .is_some()
    }
}
