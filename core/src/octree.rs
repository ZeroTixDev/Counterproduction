use crate::geometry::IVec;

pub trait OctreeSet {
    type Node: OctreeNode;
    type Iter: Iterator<Item = Self::Node>;
    fn root(&self) -> Self::Node;
    fn children(&self, node: Self::Node) -> Self::Iter;
}
pub trait OctreeNode: Copy {
    /// The bottom corner of the octree node.
    fn position(self) -> IVec;
    /// The size of the entire node.
    /// This would be 1 for a unit node.
    /// This must be a power of 2.
    fn size(self) -> u64;
    fn is_full(self) -> bool;
    fn is_unit(self) -> bool {
        self.size() == 1
    }
}
/* Implementations */
pub mod octree_set;
