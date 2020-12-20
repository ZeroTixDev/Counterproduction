use super::*;
use crate::geometry::IVec;
use building_blocks::prelude::*;
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering;

static LAST_CHUNK_INDEX: AtomicU32 = AtomicU32::new(0);

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub struct ChunkIndex(u32);

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
    fn get(&mut self) -> &T {
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

impl<T: Eq + Copy> VoxelStorage for ChunkStorage<T> {
    type T = T;
    type Position = IVec;
    type Mutator<'a> = Mutator<'a, T>;
    type PositionIterator = PositionIterator;

    fn get(&self, position: Self::Position) -> &T {
        self.map.get_ref(&convert_to_point(position))
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
    fn ambient(&self) -> T {
        self.map.ambient_value()
    }
}
impl<T: Eq + Copy> IndexableVoxelStorage for ChunkStorage<T> {
    type Index = (ChunkIndex, IVec);
    fn index_of(&self, position: Self::Position) -> Option<Self::Index> {
        self.map
            .get_chunk_containing_point(&convert_to_point(position))
            .map(|(p, c)| (c.metadata, position - convert_from_point(p)))
    }
}
impl<T: Eq + Copy> ChunkStorage<T> {
    pub fn new(ambient: T, chunk_size: i32) -> Self {
        let builder = ChunkMapBuilder {
            chunk_shape: PointN([chunk_size; 3]),
            ambient_value: ambient,
            default_chunk_metadata: ChunkIndex(0),
        };
        let map = builder.build_with_hash_map_storage();
        ChunkStorage { map }
    }
}

#[test]
fn test_chunk_storage() {
    let mut storage = ChunkStorage::new(0, 16);
    test_storage(&mut storage, 1, IVec::new(0, 0, 0));
    test_storage(&mut storage, 1, IVec::new(0, 20, 0));
    test_storage(&mut storage, 1, IVec::new(18, 93, -3));
}

impl_index!(ChunkStorage, T);
