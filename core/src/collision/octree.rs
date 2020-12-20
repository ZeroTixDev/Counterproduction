use super::*;
use crate::collision::cube::collide_cube_sloppy;
use crate::collision::cube::Cube;
use crate::geometry::IVec;
use crate::geometry::Isometry;
use crate::storage::CollidableVoxelGrid;
use building_blocks::storage::octree::OctreeNode;
use building_blocks::storage::octree::OctreeSet as Octree;
use building_blocks::storage::octree::OffsetTable;
use either::*;
use fnv::FnvHashMap;

pub struct OctreeCollisionResolver;

impl CollisionResolver for OctreeCollisionResolver {
    type Collider = Octree;
    type Position = IVec;
    fn collide<
        T: Eq + Copy,
        X: CollidableVoxelGrid<Collider = Self::Collider, Position = Self::Position>,
    >(
        collidables: impl Iterator<Item = Positioned<Self::Collider>>,
    ) -> VoxelCollisionList<Self::Position> {
        let collidables = collidables.collect::<Vec<_>>();
        let mut map: VoxelCollisionListInterior<IVec> = FnvHashMap::default();
        for bi in 0..collidables.len() {
            for ai in 0..bi {
                let a = &collidables[ai];
                let b = &collidables[bi];
                let collisions = collide_octree(a, b);
                if !collisions.is_empty() {
                    map.insert((ai, bi), collisions);
                }
            }
        }
        VoxelCollisionList { data: map }
    }
}

type OctreeCollision = (OctreeNode, OctreeNode, CollisionResult);

pub fn collide_octree(
    a: &Positioned<Octree>,
    b: &Positioned<Octree>,
) -> Vec<(IVec, IVec, CollisionResult)> {
    debug_assert!(a.object.edge_length() == b.object.edge_length());
    fn collision_cube(x: OctreeNode, e: &Positioned<Octree>) -> Positioned<Cube> {
        let mut transform = Isometry::new(e.position, e.rotation);
        let size = (x.octant().edge_length() as f32) / 2.0;
        let min_vec = x.octant().minimum().0;
        let min_vec: FVec = FVec::new(min_vec[0] as f32, min_vec[1] as f32, min_vec[2] as f32);
        transform.append_translation(min_vec + FVec::broadcast(size));
        Positioned {
            object: Cube { size },
            position: transform.translation,
            rotation: transform.rotation,
        }
    }
    let root_nodes = (a.object.root_node().unwrap(), b.object.root_node().unwrap());
    let initial_collision = collide_cube_sloppy(
        collision_cube(root_nodes.0, a),
        collision_cube(root_nodes.1, b),
    );
    if !initial_collision.collided {
        return vec![];
    }
    let root_collisions = vec![(root_nodes.0, root_nodes.1, initial_collision)];
    let offset_table = a.object.offset_table();

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

    fn collide_level(
        current: Vec<OctreeCollision>,
        entities: (&Positioned<Octree>, &Positioned<Octree>),
        current_entity_second: bool,
        table: &OffsetTable,
    ) -> Either<Vec<OctreeCollision>, OctreeCollision> {
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
                    let a_child = entities.0.object.get_child(&table, &a, i).unwrap();
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
    for _ in 0..a.object.power() * 2 {
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
        .into_iter()
        .map(|(a, b, c)| {
            (
                a.octant().minimum().0.into(),
                b.octant().minimum().0.into(),
                c,
            )
        })
        .collect()
}
