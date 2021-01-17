use bevy::ecs::Commands;
use wgpu::*;
use crate::types::*;

pub fn create_buffers(commands: &mut Commands, device: &Device) {
    let vertex_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("Vertex Buffer"),
        size: MAX_VOXELS,
        usage: BufferUsage::VERTEX,
        mapped_at_creation: false,
    });

    let entity_rotation_texture = device.create_texture(&TextureDescriptor {
        label: Some("Entity Rotation Texture"),
        size: Extent3d {
            width: MAX_ENTITIES,
            height: 1,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D1,
        format: TextureFormat::Rgba32Float,
        usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST
    });

    let entity_position_texture = device.create_texture(&TextureDescriptor {
        label: Some("Entity Position Texture"),
        size: Extent3d {
            width: MAX_ENTITIES,
            height: 1,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D1,
        format: TextureFormat::Rgba32Float,
        usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST
    });

    let type_color_texture = device.create_texture(&TextureDescriptor {
        label: Some("Type Color Texture"),
        size: Extent3d {
            width: MAX_ENTITIES,
            height: 1,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D1,
        format: TextureFormat::Rgba32Float,
        usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST
    });

    commands
        .insert_resource(VertexBuffer(vertex_buffer))
        .insert_resource(VertexBufferLength(0))
        .insert_resource(EntityRotationTexture(entity_rotation_texture))
        .insert_resource(EntityPositionTexture(entity_position_texture))
        .insert_resource(EntityTextureLength(0))
        .insert_resource(TypeColorTexture(type_color_texture));
}
