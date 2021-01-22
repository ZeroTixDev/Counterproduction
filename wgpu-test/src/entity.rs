use bevy::ecs::Commands;
use bevy::ecs::Res;
use counterproduction_core::geometry::FVec;
use counterproduction_core::geometry::Rot;
use wgpu::*;

/// An entity.
// IMPORTANT: THIS DOES NOT MATCH WITH THE TEXTURES. POSITION NEEDS PADDING.
pub struct Entity {
    pub rotation: Rot,
    pub position: FVec,
}
pub(crate) const MAX_ENTITIES: u32 = 16;

pub(crate) struct EntityRotationTexture(pub Texture);
pub(crate) struct EntityPositionTexture(pub Texture);

pub fn init(commands: &mut Commands, device: Res<Device>) {
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

    commands
        .insert_resource(EntityRotationTexture(entity_rotation_texture))
        .insert_resource(EntityPositionTexture(entity_position_texture));
}
