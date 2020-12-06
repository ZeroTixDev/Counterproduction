use crate::entity::Entity;
use crate::geometry::Isometry;
use crate::geometry::Vec3;
use crate::storage::IndexableVoxelStorage;
use crate::storage::OctreeNode;
use crate::voxel::Voxel;
use building_blocks::storage::octree::OffsetTable;
use derive_new::*;
use either::*;

#[derive(new, Copy, Clone)]
pub struct Cube {
    /// Half of the length, width, and height of the cube.
    pub size: f32,
    /// The transform of the cube.
    /// If this is the identity transform, the cube is positioned with the
    /// center being the origin.
    pub transform: Isometry,
}

pub struct CollisionResult {
    pub collided: bool,
    pub penetration: f32,
}

/// Calculates whether two cubes collide.
#[inline]
pub fn collide_cube(a: Cube, b: Cube) -> CollisionResult {
    collide_cube_sloppy(a, b)
}

/// A sloppy algorithm that calculates whether two cubes collide.
/// May provide false positives.
#[inline]
pub fn collide_cube_sloppy(a: Cube, b: Cube) -> CollisionResult {
    let a_position = a.transform.translation;
    let b_position = b.transform.translation;
    let dist = (a_position - b_position).mag();
    let max_dist = 3.0f32.sqrt() * (a.size + b.size);
    CollisionResult {
        collided: dist < max_dist,
        penetration: max_dist - dist,
    }
}

/// A collision function for two Entities.
/// Returns a vector of all blocks within
/// each Entity that collided, including
/// the collision result.

type OctreeCollisionList = Vec<(OctreeNode, OctreeNode, CollisionResult)>;

pub fn collide_entity<T: IndexableVoxelStorage<Voxel>>(
    a: &Entity<T>,
    b: &Entity<T>,
) -> OctreeCollisionList {
    debug_assert!(a.tree.edge_length() == b.tree.edge_length());
    fn collision_cube<S: IndexableVoxelStorage<Voxel>>(x: OctreeNode, e: &Entity<S>) -> Cube {
        let mut transform = e.transform;
        let size = (x.octant().edge_length() as f32) / 2.0;
        let min_vec = x.octant().minimum().0;
        let min_vec: Vec3 = Vec3::new(min_vec[0] as f32, min_vec[1] as f32, min_vec[2] as f32);
        transform.append_translation(min_vec + Vec3::broadcast(size));
        Cube { size, transform }
    }
    let root_nodes = (a.tree.root_node().unwrap(), b.tree.root_node().unwrap());
    let initial_collision = collide_cube_sloppy(
        collision_cube(root_nodes.0, a),
        collision_cube(root_nodes.1, b),
    );
    if !initial_collision.collided {
        return vec![];
    }
    let root_collisions = vec![(root_nodes.0, root_nodes.1, initial_collision)];
    let offset_table = a.tree.offset_table();

    fn swap<S>(x: (S, S), swap: bool) -> (S, S) {
        if swap {
            (x.1, x.0)
        } else {
            x
        }
    }
    fn swap_first<S, R>(x: (S, S, R), swap: bool) -> (S, S, R) {
        if swap {
            (x.1, x.0, x.2)
        } else {
            x
        }
    }

    fn collide_level<S: IndexableVoxelStorage<Voxel>>(
        current: OctreeCollisionList,
        entities: (&Entity<S>, &Entity<S>),
        current_entity_second: bool,
        table: &OffsetTable,
    ) -> Either<OctreeCollisionList, (OctreeNode, OctreeNode, CollisionResult)> {
        let mut non_leaf = vec![];
        let entities = swap(entities, current_entity_second);
        for tuple in current.into_iter() {
            let (a, b, collision) = swap_first(tuple, current_entity_second);
            if a.is_leaf() && b.is_leaf() {
                return Right(swap_first((a, b, collision), current_entity_second));
            } else if a.is_leaf() {
                non_leaf.push(swap_first((a, b, collision), current_entity_second));
            } else {
                let b_cube = collision_cube(b, entities.1);
                for i in 0..8 {
                    let a_child = entities.0.tree.get_child(&table, &a, i).unwrap();
                    let a_child_cube = collision_cube(a_child, entities.0);
                    let collision = collide_cube_sloppy(a_child_cube, b_cube);
                    if collision.collided {
                        non_leaf.push(swap_first((a_child, b, collision), current_entity_second));
                    }
                }
            }
        }
        Left(non_leaf)
    }

    let mut all_collisions = vec![];
    let mut last_collisions = root_collisions;
    let entities = (a, b);
    let mut current_entity_second = false;
    for _ in 0..a.tree.power() * 2 {
        current_entity_second = !current_entity_second;
        let mut next_collisions = vec![];
        match collide_level(
            last_collisions,
            entities,
            current_entity_second,
            &offset_table,
        ) {
            Left(collisions) => next_collisions.extend(collisions),
            Right(leaf_collision) => all_collisions.push(leaf_collision),
        }

        last_collisions = next_collisions;
    }
    all_collisions
}
