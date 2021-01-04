#![allow(clippy::type_complexity)]
#![allow(dead_code)]
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::mesh::VertexAttributeValues;
use bevy::render::pipeline::PrimitiveTopology;
use bevy::tasks::ComputeTaskPool;
use bevy::tasks::TaskPool;
use bevy_orbit_controls::*;
use building_blocks::mesh::*;
use building_blocks::prelude::*;
use counterproduction_core::collision::{octree::OctreeCollisionResolver, *};
use counterproduction_core::for_each::ForEach;
use counterproduction_core::geometry::FVec;
use counterproduction_core::geometry::IVec;
use counterproduction_core::geometry::Rot;
use counterproduction_core::octree::octree_set::BBOctreeSet;
use counterproduction_core::physics::Position;
use counterproduction_core::physics::*;
use itertools::Itertools;

use counterproduction_core::storage::chunk_map::ChunkStorage;
use counterproduction_core::storage::*;
use voxel::*;

mod voxel;

#[bevy_main]
fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(OrbitCameraPlugin)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(startup.system())
        .add_startup_system(startup_create_storage.system())
        .add_system(display_sync_transform_system.system())
        .add_system(auto_mesh_system.system())
        .add_system(octree_generator.system())
        // .add_system(apply_force_to_entity.system())
        .add_system(octree_collide.system())
        .run();
}

fn startup(commands: &mut Commands) {
    commands
        // light
        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0) * 10.0),
            ..Default::default()
        })
        // camera
        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 1.0))
                .looking_at(Vec3::default(), Vec3::unit_y()),
            ..Default::default()
        })
        .with(OrbitCamera {
            x: 0.0,
            y: 0.0,
            distance: 40.0,
            center: Vec3::zero(),
            rotate_sensitivity: 1.0,
            zoom_sensitivity: 0.8,
        });
}

fn startup_create_storage(
    commands: &mut Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    {
        let mut storage = ChunkStorage::new(Empty.into(), 16);
        cube(&mut storage, IVec::zero(), 5);
        let physics = PhysicsBundle::new(
            FVec::zero(),
            Rot::identity(),
            FVec::zero(),
            storage.for_each_map(|(pos, v)| (pos, v.mass())),
        );
        commands
            .spawn((
                VoxelMaterial(materials.add(StandardMaterial {
                    albedo: Color::rgb_u8(54, 75, 110),
                    ..Default::default()
                })),
                storage,
                ChunkMeshes(vec![]),
                GlobalTransform::default(),
            ))
            .with_bundle(physics);
    }
    {
        let mut storage = ChunkStorage::new(Empty.into(), 16);
        cube(&mut storage, IVec::zero(), 5);
        let physics = PhysicsBundle::new(
            FVec::new(40.0, 1.0, 0.0),
            Rot::identity(),
            FVec::new(-3.0, 0.0, 0.0),
            storage.for_each_map(|(pos, v)| (pos, v.mass())),
        );
        commands
            .spawn((
                VoxelMaterial(materials.add(StandardMaterial {
                    albedo: Color::rgb_u8(110, 54, 75),
                    ..Default::default()
                })),
                storage,
                ChunkMeshes(vec![]),
                GlobalTransform::default(),
            ))
            .with_bundle(physics);
    }
}

fn apply_force_to_entity(mut query: Query<(&mut Force, &mut Torque, &Position)>) {
    for mut q in query.iter_mut() {
        apply_force_bundle(
            FVec::new(1.0, 0.0, 0.0) * 20.0,
            q.2 .0 + FVec::new(0.0, 0.0, 2.0),
            &mut q,
        );
        apply_force_bundle(
            FVec::new(-1.0, 0.0, 0.0) * 20.0,
            q.2 .0 + FVec::new(0.0, 0.0, -2.0),
            &mut q,
        );
    }
}
#[allow(clippy::many_single_char_names)]
fn octree_collide(
    mut query: Query<(
        Entity,
        &BBOctreeSet,
        &mut Force,
        &mut Torque,
        &Position,
        &Rotation,
    )>,
) {
    let mut v = query.iter_mut().collect::<Vec<_>>();
    for (i, j) in (0..v.len()).tuple_combinations() {
        assert!(i < j);
        let (l, r) = v.split_at_mut(j);
        let (e1, o1, ref mut f1, ref mut t1, p1, r1) = l[i];
        let (e2, o2, ref mut f2, ref mut t2, p2, r2) = r[0];
        let x = Positioned::new(o1, p1.0, r1.0);
        let y = Positioned::new(o2, p2.0, r2.0);
        let collisions = OctreeCollisionResolver::collide(x, y);
        if !collisions.is_empty() {
            println!(
                "Entities {:?} and {:?} have collided: {:?}",
                e1,
                e2,
                collisions.len()
            );
            for (a_pos, b_pos, penetration) in collisions {
                apply_collision(
                    (f1, t1, p1, r1),
                    (f2, t2, p2, r2),
                    FVec::new(a_pos.x as f32, a_pos.y as f32, a_pos.z as f32),
                    FVec::new(b_pos.x as f32, b_pos.y as f32, b_pos.z as f32),
                    penetration,
                );
            }
        }
    }
}
fn octree_generator(
    commands: &mut Commands,
    query: Query<(Entity, &ChunkStorage<SimpleVoxel>), Changed<ChunkStorage<SimpleVoxel>>>,
) {
    for (e, storage) in query.iter() {
        commands.insert_one(e, BBOctreeSet::from_chunk_storage(storage));
    }
}

fn display_sync_transform_system(
    commands: &mut Commands,
    query: Query<(Entity, &Position, &Rotation, &CenterOfMass)>,
) {
    for (e, &Position(s), &Rotation(r), &CenterOfMass(com)) in query.iter() {
        commands.insert_one(
            e,
            Transform {
                translation: (*(s - r * com).as_array()).into(),
                rotation: Quat::from_xyzw(r.bv.yz, r.bv.xz, r.bv.xy, r.s), // TODO: FIX
                scale: Vec3::one(),
            },
        );
    }
}

fn cube(
    storage: &mut impl VoxelStorage<T = SimpleVoxel, Position = IVec>,
    center: IVec,
    size: i32,
) {
    for a in -size..=size {
        for b in -size..=size {
            for c in -size..=size {
                let pos = center + IVec::new(a, b, c);
                *storage.get_mut(pos).get_mut() = Solid.into();
            }
        }
    }
}

fn cube_rand(
    storage: &mut impl VoxelStorage<T = SimpleVoxel, Position = IVec>,
    center: IVec,
    size: i32,
    chance_filled: f32,
) {
    for a in -size..=size {
        for b in -size..=size {
            for c in -size..=size {
                let pos = center + IVec::new(a, b, c);
                if rand::random::<f32>() < chance_filled {
                    *storage.get_mut(pos).get_mut() = Solid.into();
                }
            }
        }
    }
}

struct ChunkMeshes(Vec<Entity>);
struct VoxelMaterial(Handle<StandardMaterial>);

fn auto_mesh_system(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    pool: Res<ComputeTaskPool>,
    mut query: Query<
        (
            Entity,
            &VoxelMaterial,
            &ChunkStorage<SimpleVoxel>,
            &mut ChunkMeshes,
        ),
        Changed<ChunkStorage<SimpleVoxel>>,
    >,
) {
    for (e, VoxelMaterial(material), storage, mut prev_meshes) in query.iter_mut() {
        for a in prev_meshes.0.iter() {
            commands.despawn(*a);
        }
        let chunk_meshes = generate_meshes(&storage.map, &pool.0);
        let chunk_meshes = chunk_meshes
            .into_iter()
            .map(|m| create_mesh_entity(e, m, commands, material.clone(), &mut meshes))
            .collect::<Vec<_>>();
        *prev_meshes = ChunkMeshes(chunk_meshes);
    }
}
fn create_mesh_entity(
    parent: Entity,
    mesh: PosNormMesh,
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    meshes: &mut Assets<Mesh>,
) -> Entity {
    assert_eq!(mesh.positions.len(), mesh.normals.len());
    let num_vertices = mesh.positions.len();

    let mut render_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    render_mesh.set_attribute(
        "Vertex_Position",
        VertexAttributeValues::Float3(mesh.positions),
    );
    render_mesh.set_attribute("Vertex_Normal", VertexAttributeValues::Float3(mesh.normals));
    render_mesh.set_attribute(
        "Vertex_Uv",
        VertexAttributeValues::Float2(vec![[0.0; 2]; num_vertices]),
    );
    render_mesh.set_indices(Some(Indices::U32(mesh.indices)));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(render_mesh),
            material,
            ..Default::default()
        })
        .with(Parent(parent))
        .current_entity()
        .unwrap()
}
fn generate_meshes<M: Sync>(
    map_ref: &ChunkHashMap3<SimpleVoxel, M>,
    pool: &TaskPool,
) -> Vec<PosNormMesh> {
    let res = pool.scope(|s| {
        for chunk_key in map_ref.storage().keys() {
            s.spawn(async move {
                let padded_chunk_extent = padded_greedy_quads_chunk_extent(
                    &map_ref.indexer.extent_for_chunk_at_key(*chunk_key),
                );

                let mut padded_chunk = Array3::fill(padded_chunk_extent, Empty.into());
                copy_extent(&padded_chunk_extent, map_ref, &mut padded_chunk);

                let mut buffer = GreedyQuadsBuffer::new(padded_chunk_extent);
                greedy_quads(&padded_chunk, &padded_chunk_extent, &mut buffer);

                let mut mesh = PosNormMesh::default();
                for group in buffer.quad_groups.iter() {
                    for (quad, _material) in group.quads.iter() {
                        group.face.add_quad_to_pos_norm_mesh(&quad, &mut mesh);
                    }
                }

                for [x, y, z] in mesh.positions.iter_mut() {
                    *x -= 0.5;
                    *y -= 0.5;
                    *z -= 0.5;
                }

                if mesh.is_empty() {
                    None
                } else {
                    Some(mesh)
                }
            })
        }
    });
    res.into_iter().filter_map(|x| x).collect()
}
