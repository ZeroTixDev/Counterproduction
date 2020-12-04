use crate::geometry::Isometry;
use derive_new::*;

#[derive(new)]
pub struct Cube {
    /// Half of the length, width, and height of the cube.
    pub size: f32,
    /// The transform of the cube.
    /// If this is the identity transform, the cube is positioned with the center being the origin.
    pub transform: Isometry,
}

/// Calculates whether two cubes collide.
#[inline]
pub fn collide_cube(a: Cube, b: Cube) -> bool {
    collide_cube_sloppy(a, b)
}

/// A sloppy algorithm that calculates whether two cubes collide.
/// May provide false positives.
#[inline]
pub fn collide_cube_sloppy(a: Cube, b: Cube) -> bool {
    let a_position = a.transform.translation;
    let b_position = b.transform.translation;
    let dist_squared = (a_position - b_position).mag_sq();
    let max_dist_squared = 3.0 * (a.size + b.size) * (a.size + b.size);
    dist_squared < max_dist_squared
}
