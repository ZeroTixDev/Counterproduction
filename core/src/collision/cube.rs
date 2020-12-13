use super::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Cube {
    /// Half of the length, width, and height of the cube.
    pub size: f32,
}

/// Calculates whether two cubes collide.
#[inline]
pub fn collide_cube(a: Positioned<Cube>, b: Positioned<Cube>) -> CollisionResult {
    collide_cube_sloppy(a, b)
}

/// A sloppy algorithm that calculates whether two cubes collide.
/// May provide false positives.
#[inline]
pub fn collide_cube_sloppy(a: Positioned<Cube>, b: Positioned<Cube>) -> CollisionResult {
    let a_position = a.position;
    let b_position = b.position;
    let dist = (a_position - b_position).mag();
    let max_dist = 3.0f32.sqrt() * (a.object.size + b.object.size);
    CollisionResult {
        penetration: max_dist - dist,
        collided: dist < max_dist,
    }
}
