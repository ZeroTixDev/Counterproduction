use super::*;

const CUBE_SIZE_FACTOR: f32 = 0.9;

#[derive(new, Copy, Clone, PartialEq, Debug)]
pub struct Cube {
    /// Half of the length, width, and height of the cube.
    pub size: f32,
}

/// Calculates whether two cubes collide.
pub fn collide_cube(a: Positioned<Cube>, b: Positioned<Cube>) -> CollisionResult {
    let a_new = Positioned {
        object: Cube::new(a.object.size * CUBE_SIZE_FACTOR),
        position: a.position,
        rotation: a.rotation,
    };
    let b_new = Positioned {
        object: Cube::new(b.object.size * CUBE_SIZE_FACTOR),
        position: b.position,
        rotation: b.rotation,
    };
    collide_cube_sloppy(a_new, b_new)
}

/// A sloppy algorithm that calculates whether two cubes collide.
/// May provide false positives.
pub fn collide_cube_sloppy(a: Positioned<Cube>, b: Positioned<Cube>) -> CollisionResult {
    let delta = a.position - b.position;
    // println!("{:?} | {:?}", a.position, b.position);
    let dist = delta.mag();
    let max_dist = 3.0f32.sqrt() * (a.object.size + b.object.size);
    CollisionResult {
        penetration: (max_dist - dist) * delta.normalized(),
        collided: dist < max_dist,
    }
}

#[test]
fn test_collide_cube() {
    let object = Cube::new(1.0);
    let cube1 = Positioned {
        object,
        position: FVec::zero(),
        rotation: Rot::identity(),
    };
    let cube2 = Positioned {
        object,
        position: FVec::new(3.5, 0.0, 0.0),
        rotation: Rot::identity(),
    };
    assert_eq!(collide_cube(cube1, cube2).collided, false);
    let cube3 = Positioned {
        object,
        position: FVec::new(2.1, 2.1, 2.1),
        rotation: Rot::identity(),
    };
    assert_eq!(collide_cube(cube1, cube3).collided, false);
    let cube4 = Positioned {
        object,
        position: FVec::new(1.0, 1.0, 1.0),
        rotation: Rot::identity(),
    };
    assert_eq!(collide_cube(cube1, cube4).collided, true);
    let cube5 = Positioned {
        object: Cube::new(0.5),
        position: FVec::new(1.6, 1.6, 1.6),
        rotation: Rot::identity(),
    };
    assert_eq!(collide_cube(cube1, cube5).collided, false);
}
