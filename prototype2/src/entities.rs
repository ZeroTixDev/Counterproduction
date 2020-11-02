use bevy::prelude::*;
use shape::Cube;
use derive_new::*;

pub struct EntityPlugin;

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .add_startup_system(initialize_materials.system())
            .add_system(fill_entity.system())
            .add_system(fill_mesh.system());
    }
}

fn initialize_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    let body = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
    let gun = materials.add(Color::rgb(0.6, 0.7, 0.5).into());
    commands.insert_resource(Materials { body, gun });
}

fn fill_entity(mut commands: Commands, query: Query<Without<Health, (Entity, &Stats, &Unit)>>) {
    for x in query.iter() {
        commands.insert_one(x.0, Health(x.1.health.0));
    }
}
fn fill_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: Res<Materials>,
    query: Query<Without<Mesh, (Entity, &Stats, &Position, &Unit)>>,
) {
    for x in query.iter() {
        commands
            .insert(
                x.0,
                PbrComponents {
                    material: materials.body.clone(),
                    mesh: meshes.add(Mesh::from(Cube { size : 1.0 })),
                    transform: Transform::from_translation(x.2.0),
                    ..Default::default()
                },
            )
            .with_children(|parent| {
                parent.spawn(PbrComponents {
                    material: materials.gun.clone(),
                    mesh: meshes.add(Mesh::from(Cube { size: 0.5 })),
                    transform: Transform::from_translation(Vec3::new(1.0, 0.0, 0.0)),
                    ..Default::default()
                });
            });
    }
}

struct Materials {
    pub body: Handle<StandardMaterial>,
    pub gun: Handle<StandardMaterial>,
}

pub struct Unit;

#[derive(Bundle)]
pub struct UnitProps {
    pub stats: Stats,
    pub position: Position,
    pub unit: Unit,
}

impl UnitProps {
    pub fn new(position: Vec3, stats: Stats) -> Self {
        UnitProps { position: Position(position), stats, unit: Unit }
    }
}

#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct Health(pub f64);
#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct Position(pub Vec3);

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Stats {
    pub health: Bounded<0.0, 10.0>,
    pub firepower: Bounded<0.0, 2.0>,
    pub range: Bounded<0.0, 10.0>,
    pub movement: Bounded<0.0, 2.0>,
}

impl Stats {
    pub fn new(h: f64, f: f64, r: f64, m: f64) -> Self {
        Stats {
            health: Bounded::new(h),
            firepower: Bounded::new(f),
            range: Bounded::new(r),
            movement: Bounded::new(m),
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct Bounded<const MIN: f64, const MAX: f64>(pub f64);

impl<const MIN: f64, const MAX: f64> Bounded<{ MIN }, { MAX }> {
    pub fn new(x: f64) -> Self {
        if x < MIN || x >= MAX {
            panic!("Value out of bounds: {}", x);
        }
        Bounded(x)
    }
    pub fn min(self) -> f64 {
        MIN
    }
    pub fn max(self) -> f64 {
        MAX
    }
}
