use crate::geometry::IVec;
use crate::octree::OctreeNode as OctreeNodeTrait;
use crate::octree::OctreeSet as OctreeSetTrait;
use crate::storage::chunk_map::ChunkStorage;
use crate::storage::VoxelStorage;
use building_blocks::prelude::IsEmpty;
use building_blocks::prelude::*;
use building_blocks::storage::{OctreeNode, OctreeSet, OffsetTable};

pub struct BBOctreeSet {
    pub set: OctreeSet,
    table: OffsetTable,
}

impl BBOctreeSet {
    pub fn new(set: OctreeSet) -> Self {
        let table = set.offset_table();
        BBOctreeSet { set, table }
    }
    pub fn from_chunk_storage<T: IsEmpty + Eq + Copy>(storage: &ChunkStorage<T>) -> Self {
        fn next_pow(a: i32) -> i32 {
            (a as u32).next_power_of_two() as i32
        }
        let map = &storage.map;
        let mut extent = map.bounding_extent();
        let shape = extent.shape.0;
        extent.shape = PointN([next_pow(shape[0]), next_pow(shape[1]), next_pow(shape[2])]);
        let mut array = Array3::fill(extent, storage.ambient());
        copy_extent(&extent, map, &mut array);
        BBOctreeSet::new(OctreeSet::from_array3(&array, extent))
    }
}

impl OctreeSetTrait for BBOctreeSet {
    type Node = BBOctreeNode;
    type Iter = std::vec::IntoIter<Self::Node>;
    fn root(&self) -> Self::Node {
        BBOctreeNode::Node(self.set.root_node().unwrap())
    }
    fn children(&self, node: Self::Node) -> Self::Iter {
        let mut out_vec = vec![];
        match node {
            BBOctreeNode::Node(node) => {
                for i in 0..8 {
                    if let Some(child_node) = self.set.get_child(&self.table, &node, i) {
                        if child_node.is_leaf() {
                            out_vec.push(BBOctreeNode::Full(
                                child_node.octant().minimum().0.into(),
                                1 << child_node.power(),
                            ));
                        } else {
                            out_vec.push(BBOctreeNode::Node(child_node));
                        }
                    }
                }
            }
            BBOctreeNode::Full(pos, size) => {
                if size > 1 {
                    let half_size = size / 2;
                    for i in 0..2 {
                        for j in 0..2 {
                            for k in 0..2 {
                                out_vec.push(BBOctreeNode::Full(
                                    pos + IVec::new(i, j, k) * half_size as i32,
                                    half_size,
                                ));
                            }
                        }
                    }
                }
            }
        }
        out_vec.into_iter()
    }
}
#[derive(Clone, Copy)]
pub enum BBOctreeNode {
    Node(OctreeNode),
    Full(IVec, u64),
}
impl OctreeNodeTrait for BBOctreeNode {
    fn position(self) -> IVec {
        match self {
            BBOctreeNode::Node(node) => node.octant().minimum().0.into(),
            BBOctreeNode::Full(pos, _) => pos,
        }
    }
    fn size(self) -> u64 {
        match self {
            BBOctreeNode::Node(node) => 1 << node.power(),
            BBOctreeNode::Full(_, size) => size,
        }
    }
    fn is_full(self) -> bool {
        match self {
            BBOctreeNode::Node(node) => node.is_leaf(),
            BBOctreeNode::Full(..) => true,
        }
    }
}
