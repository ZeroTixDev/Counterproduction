use bevy::prelude::*;
use enum_dispatch::*;

#[enum_dispatch]
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum SimpleVoxelType {
    Empty,
    Solid,
}
#[enum_dispatch(SimpleVoxelType)]
pub trait SimpleVoxel: Copy + Eq {
    fn color(self) -> Color;
    fn collidable(self) -> bool;
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Empty;
impl SimpleVoxel for Empty {
    fn color(self) -> Color {
        Color::rgba(0.0, 0.0, 0.0, 0.0)
    }
    fn collidable(self) -> bool {
        false
    }
}
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct Solid;
impl SimpleVoxel for Solid {
    fn color(self) -> Color {
        Color::rgb(0.22, 0.27, 0.35)
    }
    fn collidable(self) -> bool {
        true
    }
}
