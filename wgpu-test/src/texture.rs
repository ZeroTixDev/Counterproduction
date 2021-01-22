use crate::entity::EntityPositionTexture;
use crate::entity::EntityRotationTexture;
use crate::type_color::TypeColorTexture;
use bevy::ecs::Commands;
use bevy::ecs::Res;

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

pub(crate) struct TextureBindGroupLayout(pub BindGroupLayout);
pub(crate) struct TextureBindGroup(pub BindGroup);

pub(crate) fn init(
    commands: &mut Commands,
    device: Res<Device>,
    entity_rotation_texture: Res<EntityRotationTexture>,
    entity_position_texture: Res<EntityPositionTexture>,
    type_color_texture: Res<TypeColorTexture>,
) {
    let entity_rotation_view_sampler = texture_view_sampler(&device, &entity_rotation_texture.0);
    let entity_position_view_sampler = texture_view_sampler(&device, &entity_position_texture.0);
    let type_color_view_sampler = texture_view_sampler(&device, &type_color_texture.0);

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

    commands
        .insert_resource(TextureBindGroupLayout(bind_group_layout))
        .insert_resource(TextureBindGroup(bind_group));
}
