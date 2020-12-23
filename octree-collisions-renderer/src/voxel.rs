use bevy::prelude::*;
use building_blocks::mesh::MaterialVoxel;
use building_blocks::prelude::IsEmpty;
use enum_dispatch::*;

#[enum_dispatch]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SimpleVoxel {
    Empty,
    Solid,
}
#[enum_dispatch(SimpleVoxel)]
pub trait SimpleVoxelType: Copy + Eq {
    fn color(self) -> Color;
    fn collidable(self) -> bool;
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Empty;
impl SimpleVoxelType for Empty {
    fn color(self) -> Color {
        Color::rgba(0.0, 0.0, 0.0, 0.0)
    }
    fn collidable(self) -> bool {
        false
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Solid;
impl SimpleVoxelType for Solid {
    fn color(self) -> Color {
        Color::rgb(0.22, 0.27, 0.35)
    }
    fn collidable(self) -> bool {
        true
    }
}
impl IsEmpty for SimpleVoxel {
    fn is_empty(&self) -> bool {
        *self == SimpleVoxel::Empty(Empty)
    }
}
impl MaterialVoxel for SimpleVoxel {
    type Material = ();
    fn material(&self) {}
}
