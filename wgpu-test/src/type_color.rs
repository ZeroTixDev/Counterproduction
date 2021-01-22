use crate::color::LinRgbaColor;
use crate::color::RgbaColor;
use bevy::ecs::Commands;
use bevy::ecs::Res;
use bytemuck::{cast_slice, Pod, Zeroable};
use std::mem::size_of;

use wgpu::*;

pub struct TypeColors<const TYPES: usize>(pub [RgbaColor; TYPES]);

pub(crate) struct TypeColorTexture(pub Texture);

#[derive(Copy, Clone, PartialEq, Debug)]
struct LinRgbaColorWrapper(LinRgbaColor);
unsafe impl Pod for LinRgbaColorWrapper {}
unsafe impl Zeroable for LinRgbaColorWrapper {}

pub fn init<const TYPES: usize>(
    commands: &mut Commands,
    device: Res<Device>,
    queue: Res<Queue>,
    type_colors: Res<TypeColors<TYPES>>,
) {
    let type_colors = type_colors.0;

    let type_color_texture = device.create_texture(&TextureDescriptor {
        label: Some("Type Color Texture"),
        size: Extent3d {
            width: TYPES as u32,
            height: 1,
            depth: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: TextureDimension::D1,
        format: TextureFormat::Rgba32Float,
        usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
    });

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
            bytes_per_row: (TYPES * size_of::<RgbaColor>()) as u32,
            rows_per_image: 0,
        },
        Extent3d {
            width: type_colors.len() as u32,
            height: 1,
            depth: 1,
        },
    );

    commands.insert_resource(TypeColorTexture(type_color_texture));
}
