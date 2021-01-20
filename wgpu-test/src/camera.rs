use counterproduction_core::geometry::*;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Camera {
    pub position: FVec,
    pub rotation: Rot,
    pub fov_ratio: f32,
    pub z_near: f32,
    pub z_far: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
#[repr(C)]
pub(crate) struct CameraRaw {
    projection_matrix: FHMat,
    position: FVec,
}
unsafe impl bytemuck::Pod for CameraRaw {}
unsafe impl bytemuck::Zeroable for CameraRaw {}
