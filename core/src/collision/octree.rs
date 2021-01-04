use super::*;
use crate::collision::cube::collide_cube;
use crate::collision::cube::collide_cube_sloppy;
use crate::collision::cube::Cube;
use crate::geometry::IVec;

use crate::octree::*;
use std::marker::PhantomData;

#[allow(clippy::type_complexity)]
pub struct OctreeCollisionResolver<'a, Set: OctreeSet>(PhantomData<(&'a (), fn(Set) -> Set)>);

impl<'a, Set: 'a + OctreeSet> CollisionResolver for OctreeCollisionResolver<'a, Set> {
    type Collider = &'a Set;
    type Position = IVec;
    fn collide(
        a: Positioned<Self::Collider>,
        b: Positioned<Self::Collider>,
    ) -> VoxelCollisionList<Self::Position> {
        let mut colliding_nodes = vec![((a.object.root(), a), (b.object.root(), b))];
        let mut voxel_collisions: VoxelCollisionList<Self::Position> = vec![];
        fn cube_from<Set: OctreeSet>(
            (node, global): (Set::Node, Positioned<&Set>),
        ) -> Positioned<Cube> {
            let half_size = node.size() as f32 / 2.0;
            let pos = node.position();
            Positioned {
                object: Cube::new(half_size),
                rotation: global.rotation,
                position: global.position
                    + global.rotation
                        * (FVec::new(pos.x as f32, pos.y as f32, pos.z as f32)
                            + FVec::one() * half_size),
            }
        }
        while !colliding_nodes.is_empty() {
            take_mut::take(&mut colliding_nodes, |nodes| {
                let mut next = vec![];
                for (mut x, mut y) in nodes.into_iter() {
                    if y.0.size() > x.0.size() {
                        std::mem::swap(&mut x, &mut y);
                    }
                    // x is always the larger one.
                    let x_collide = cube_from(x);
                    let y_collide = cube_from(y);
                    if x.0.size() == 1 {
                        let collision = collide_cube(x_collide, y_collide);
                        if collision.collided {
                            if std::ptr::eq(x.1.object, a.object) {
                                voxel_collisions.push((
                                    x.0.position(),
                                    y.0.position(),
                                    collision.penetration,
                                ))
                            } else {
                                voxel_collisions.push((
                                    y.0.position(),
                                    x.0.position(),
                                    collision.penetration,
                                ));
                            }
                        }
                    } else if collide_cube_sloppy(x_collide, y_collide).collided {
                        for a in x.1.object.children(x.0) {
                            next.push((y, (a, x.1)));
                        }
                    }
                }
                next
            })
        }
        voxel_collisions
    }
}
