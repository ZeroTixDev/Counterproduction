use bevy::ecs::Commands;
use bevy::ecs::Res;
use counterproduction_core::geometry::IVec;
use wgpu::*;

// TODO: DECIDE ON ENTITY AND UID TYPES
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Voxel {
    pub position: IVec,
    pub entity: u16,
    pub id: u16,
}

pub(crate) struct VoxelBuffer(pub Buffer);
pub(crate) struct VoxelBufferLength(pub u64);

impl Voxel {
    pub(crate) fn desc() -> VertexBufferDescriptor<'static> {
        VertexBufferDescriptor {
            stride: std::mem::size_of::<Voxel>() as BufferAddress,
            step_mode: InputStepMode::Vertex,
            attributes: &[
                VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: VertexFormat::Int3,
                },
                // Both shorts are combined into one `Short2` as there isn't a single `Short`.
                VertexAttributeDescriptor {
                    offset: std::mem::size_of::<IVec>() as BufferAddress,
                    shader_location: 1,
                    format: VertexFormat::Short2,
                },
            ],
        }
    }
}
pub(crate) const MAX_VOXELS: u32 = 1024;

pub fn init(commands: &mut Commands, device: Res<Device>) {
    let voxel_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("Voxel Buffer"),
        size: MAX_VOXELS as u64,
        usage: BufferUsage::VERTEX | BufferUsage::COPY_DST | BufferUsage::COPY_SRC,
        mapped_at_creation: false,
    });

    commands
        .insert_resource(VoxelBuffer(voxel_buffer))
        .insert_resource(VoxelBufferLength(0));
}
