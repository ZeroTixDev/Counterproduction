use enum_dispatch::*;

#[enum_dispatch]
trait VoxelType {
    fn is_collidable(self) -> bool;
}

#[enum_dispatch(VoxelType)]
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Voxel {
    Vaccum,
    Solid,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Vaccum;
impl VoxelType for Vaccum {
    #[inline]
    fn is_collidable(self) -> bool {
        false
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Solid;
impl VoxelType for Solid {
    #[inline]
    fn is_collidable(self) -> bool {
        true
    }
}
