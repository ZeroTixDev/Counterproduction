use counterproduction_core::geometry::{FVec, IVec, Rot};
use wgpu::*;
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
// TODO: DECIDE ON ENTITY AND UID TYPES
pub struct Voxel {
    pub position: IVec,
    pub entity: u16,
    pub id: u16,
}
unsafe impl bytemuck::Pod for Voxel {}
unsafe impl bytemuck::Zeroable for Voxel {}
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
/// An entity.
// IMPORTANT: THIS DOES NOT MATCH WITH THE TEXTURES. POSITION NEEDS PADDING.
pub struct Entity {
    pub rotation: Rot,
    pub position: FVec,
}
// CHANGE LATER
pub const MAX_VOXELS: u64 = 1024;
pub const MAX_ENTITIES: u32 = 16;

pub struct VertexBuffer(pub Buffer);
pub struct EntityRotationTexture(pub Texture);
pub struct EntityPositionTexture(pub Texture);
pub struct TypeColorTexture(pub Texture);
// TODO: THIS NEEDS TO BE A u32 FOR SOME REASON. FIGURE OUT WORKAROUND OR SET TO
// BE u32.
pub struct VertexBufferLength(pub u64);
pub struct EntityTextureLength(pub u64);
pub struct TextureBindGroup(pub BindGroup);

pub type RgbaColor = palette::Srgba;
pub type LinRgbaColor = palette::LinSrgba;
