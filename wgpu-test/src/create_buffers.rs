use crate::types::*;
use bevy::ecs::Commands;
use bytemuck::{cast_slice, Pod, Zeroable};
use std::mem::size_of;

use wgpu::*;

fn bind_group_layout_single(
    start: u32,
    visibility: ShaderStage,
) -> (BindGroupLayoutEntry, BindGroupLayoutEntry) {
    (
        BindGroupLayoutEntry {
            binding: start * 2,
            visibility,
            ty: BindingType::SampledTexture {
                multisampled: false,
                dimension: TextureViewDimension::D1,
                component_type: TextureComponentType::Float,
            },
            count: None,
        },
        BindGroupLayoutEntry {
            binding: start * 2 + 1,
            visibility,
            ty: BindingType::Sampler { comparison: false },
            count: None,
        },
    )
}

fn texture_view_sampler(device: &Device, texture: &Texture) -> (TextureView, Sampler) {
    let texture_view = texture.create_view(&TextureViewDescriptor::default());
    let sampler = device.create_sampler(&SamplerDescriptor {
        address_mode_u: wgpu::AddressMode::ClampToEdge,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        address_mode_w: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Nearest,
        mipmap_filter: wgpu::FilterMode::Nearest,
        ..Default::default()
    });
    (texture_view, sampler)
}

fn bind_group_single(
    start: u32,
    view_sampler: &(TextureView, Sampler),
) -> (BindGroupEntry, BindGroupEntry) {
    (
        BindGroupEntry {
            binding: start * 2,
            resource: BindingResource::TextureView(&view_sampler.0),
        },
        BindGroupEntry {
            binding: start * 2 + 1,
            resource: BindingResource::Sampler(&view_sampler.1),
        },
    )
}

pub fn create_buffers(
    commands: &mut Commands,
    device: &Device,
    queue: &Queue,
    type_colors: &[RgbaColor],
    voxels: &[Voxel],
) -> BindGroupLayout {
    let vertex_buffer = device.create_buffer(&BufferDescriptor {
        label: Some("Vertex Buffer"),
        size: MAX_VOXELS,
        usage: BufferUsage::VERTEX | BufferUsage::COPY_DST | BufferUsage::COPY_SRC,
        mapped_at_creation: false,
    });

    let entity_rotation_texture = device.create_texture(&TextureDescriptor {
        label: Some("Entity Rotation Texture"),
        size: Extent3d {
            width: MAX_ENTITIES,
            height: 3,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D2,
        format: TextureFormat::Rgba32Float,
        usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
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
        usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
    });

    let type_color_texture = device.create_texture(&TextureDescriptor {
        label: Some("Type Color Texture"),
        size: Extent3d {
            width: type_colors.len() as u32,
            height: 1,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D1,
        format: TextureFormat::Rgba32Float,
        usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
    });

    let entity_rotation_view_sampler = texture_view_sampler(device, &entity_rotation_texture);
    let entity_position_view_sampler = texture_view_sampler(device, &entity_position_texture);
    let type_color_view_sampler = texture_view_sampler(device, &type_color_texture);

    let bind_group_layout = {
        let entity_rotation_layout = bind_group_layout_single(0, ShaderStage::VERTEX);
        let entity_position_layout = bind_group_layout_single(1, ShaderStage::VERTEX);
        let type_color_layout = bind_group_layout_single(2, ShaderStage::VERTEX);
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Texture Bind Group Layout"),
            entries: &[
                entity_rotation_layout.0,
                entity_rotation_layout.1,
                entity_position_layout.0,
                entity_position_layout.1,
                type_color_layout.0,
                type_color_layout.1,
            ],
        })
    };

    let bind_group = {
        let entity_rotation = bind_group_single(0, &entity_rotation_view_sampler);
        let entity_position = bind_group_single(1, &entity_position_view_sampler);
        let type_color = bind_group_single(2, &type_color_view_sampler);
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("Texture Bind Group"),
            layout: &bind_group_layout,
            entries: &[
                entity_rotation.0,
                entity_rotation.1,
                entity_position.0,
                entity_position.1,
                type_color.0,
                type_color.1,
            ],
        })
    };

    /* ==== Write data to buffers ==== */

    println!("{:?}", type_colors);

    let type_colors_linear = &type_colors
        .iter()
        .map(|x| LinRgbaColorWrapper(x.into_linear()))
        .collect::<Vec<_>>()[..];

    queue.write_texture(
        TextureCopyView {
            texture: &type_color_texture,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
        },
        cast_slice(type_colors_linear),
        TextureDataLayout {
            offset: 0,
            bytes_per_row: (type_colors.len() * size_of::<RgbaColor>()) as u32,
            rows_per_image: 0,
        },
        Extent3d {
            width: type_colors.len() as u32,
            height: 1,
            depth: 1,
        },
    );

    queue.write_buffer(&vertex_buffer, 0, cast_slice(voxels));

    commands
        .insert_resource(VertexBuffer(vertex_buffer))
        .insert_resource(VertexBufferLength(voxels.len() as u64))
        .insert_resource(EntityRotationTexture(entity_rotation_texture))
        .insert_resource(EntityPositionTexture(entity_position_texture))
        .insert_resource(EntityTextureLength(0))
        .insert_resource(TypeColorTexture(type_color_texture))
        .insert_resource(TextureBindGroup(bind_group));

    bind_group_layout
}

#[derive(Copy, Clone, PartialEq, Debug)]
struct LinRgbaColorWrapper(LinRgbaColor);
unsafe impl Pod for LinRgbaColorWrapper {}
unsafe impl Zeroable for LinRgbaColorWrapper {}
