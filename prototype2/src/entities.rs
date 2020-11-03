use bevy::prelude::*;
use derive_new::*;
use shape::Cube;

const MOVEMENT_TOLERANCE: f32 = 1.01;
const FIRE_TOLERANCE: f32 = 1.01;

#[derive(Clone, Eq, PartialEq, Default, Debug)]
struct Materials {
    pub body: Handle<StandardMaterial>,
    pub gun: Handle<StandardMaterial>,
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Unit;

#[derive(Bundle)]
pub struct UnitProps {
    pub stats: Stats,
    pub position: Position,
    pub unit: Unit,
    pub health: Health,
}

impl UnitProps {
    pub fn new(position: Vec3, stats: Stats) -> Self {
        UnitProps {
            position: Position(Transform::from_translation(position)),
            stats,
            unit: Unit,
            health: Health(stats.health.0),
        }
    }
}

#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct Health(pub f32);
#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct Position(pub Transform);
#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct Move {
    /// The delta must be normalized to between zero and one.
    /// If it is larger, an error will be thrown.
    /// Before movement, the delta is multiplied by the movement of the entity.
    pub delta: Vec3,
}
#[derive(new, Clone, Copy, PartialEq, Debug)]
pub struct Fire {
    pub target: Entity,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Stats {
    pub health: Bounded<0.0, 10.0>,
    pub firepower: Bounded<0.0, 2.0>,
    pub range: Bounded<0.0, 10.0>,
    pub movement: Bounded<0.0, 2.0>,
}

impl Stats {
    pub fn new(h: f32, f: f32, r: f32, m: f32) -> Self {
        Stats {
            health: Bounded::new(h),
            firepower: Bounded::new(f),
            range: Bounded::new(r),
            movement: Bounded::new(m),
        }
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct Bounded<const MIN: f32, const MAX: f32>(pub f32);

impl<const MIN: f32, const MAX: f32> Bounded<{ MIN }, { MAX }> {
    pub fn new(x: f32) -> Self {
        if x < MIN || x >= MAX {
            panic!("Value out of bounds: {}", x);
        }
        Bounded(x)
    }
    pub fn min(self) -> f32 {
        MIN
    }
    pub fn max(self) -> f32 {
        MAX
    }

    pub fn between(self) -> f32 {
        (self.0 - MIN) / (MAX - MIN)
    }
}

pub struct EntityPlugin;

impl EntityPlugin {
    fn initialize_materials_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let body = materials.add(Color::rgb(0.8, 0.7, 0.6).into());
        let gun = materials.add(Color::rgb(0.6, 0.7, 0.5).into());
        commands.insert_resource(Materials { body, gun });
    }
    fn fill_mesh_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<Materials>,
        query: Query<Without<Mesh, (Entity, &Stats, &Position, &Unit)>>,
    ) {
        for x in query.iter() {
            let size = 3.0 + x.1.health.between();
            let gunsize = x.1.firepower.between() * size;
            let child = commands
                .insert(
                    x.0,
                    PbrComponents {
                        material: materials.body.clone(),
                        mesh: meshes.add(Mesh::from(Cube { size })),
                        transform: (x.2).0,
                        ..Default::default()
                    },
                )
                .spawn(PbrComponents {
                    material: materials.gun.clone(),
                    mesh: meshes.add(Mesh::from(Cube { size: gunsize })),
                    transform: Transform::from_translation(Vec3::new(
                        size - gunsize + x.1.range.between() * 2.0 * gunsize,
                        0.0,
                        0.0,
                    )),
                    ..Default::default()
                })
                .current_entity();
            commands.push_children(x.0, &[child.unwrap()]);
        }
    }
    fn move_system(
        mut commands: Commands,
        mut query: Query<(Entity, &Move, &Stats, &mut Position)>,
    ) {
        for (e, delta, stats, mut position) in query.iter_mut() {
            let delta = delta.delta;
            if delta.length_squared() > MOVEMENT_TOLERANCE {
                panic!("Movement value too large. Delta is {:?}, which has a magnitude greater than 1.", delta);
            }
            let delta = delta * stats.movement.0;
            position.0 = position.0 * Transform::from_translation(delta);
            commands.remove_one::<Move>(e);
        }
    }
    fn fire_system(
        mut commands: Commands,
        query: Query<(Entity, &Fire, &Position, &Stats)>,
        mut others: Query<(&Position, &mut Health)>,
    ) {
        for (e, fire, position, stats) in query.iter() {
            let target = fire.target;
            let firepower = stats.firepower;
            let range = stats.range;
            let mut other = others.get_mut(target).expect("Invalid Target");
            if ((other.0).0.translation - position.0.translation).length()
                > FIRE_TOLERANCE * range.0
            {
                panic!(
                    "Target at position {:?} is too far away for entity at position {:?} to fire at. Entity's stats are {:#?}",
                    (other.0).0.translation, position.0.translation, stats
                );
            }
            (other.1).0 -= firepower.0;
            commands.remove_one::<Fire>(e);
        }
    }
}
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::initialize_materials_system.system())
            .add_system(Self::fill_mesh_system.system())
            .add_system_to_stage(stage::POST_UPDATE, Self::move_system.system())
            .add_system_to_stage(stage::POST_UPDATE, Self::fire_system.system());
    }
}
