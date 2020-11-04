use bevy::prelude::*;
use derive_new::*;
use shape::Cube;

const MOVEMENT_TOLERANCE: f32 = 1.01;
const FIRE_TOLERANCE: f32 = 1.01;

#[derive(Clone, Eq, PartialEq, Default, Debug)]
struct Materials {
    pub damaged: Handle<StandardMaterial>,
    pub gun: Handle<StandardMaterial>,
}

#[derive(Clone, Eq, PartialEq, Default, Debug)]
pub struct Unit;

#[derive(Bundle)]
pub struct UnitProps {
    stats: Stats,
    position: Position,
    unit: Unit,
    health: Health,
    material: EntityColor,
}

#[derive(Clone, PartialEq, Debug)]
pub struct UnitData {
    pub stats: Stats,
    pub position: Transform,
    pub health: f32,
    pub parent: Entity,
}

impl UnitProps {
    pub fn new(position: Vec3, stats: Stats, material: Handle<StandardMaterial>) -> Self {
        UnitProps {
            position: Position(Transform::from_translation(position)),
            stats,
            unit: Unit,
            health: Health(stats.health.0),
            material: EntityColor(material),
        }
    }
}

#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct Health(pub f32);
#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
struct Position(Transform);
#[derive(new, Clone, PartialEq, Default, Debug)]
struct EntityColor(Handle<StandardMaterial>);
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
#[derive(new, Clone, Copy, PartialEq, Debug)]
struct FireAt {
    pub target: Entity,
    pub position: Vec3,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub struct Stats {
    pub health: Bounded<1.0, 10.0>,
    pub firepower: Bounded<1.0, 3.0>,
    pub range: Bounded<5.0, 50.0>,
    pub movement: Bounded<1.0, 3.0>,
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

    pub fn price(&self) -> f32 {
        0.0
    }
}

#[derive(Clone, Copy, PartialEq, PartialOrd, Debug)]
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

impl<const MIN: f32, const MAX: f32> Default for Bounded<{ MIN }, { MAX }> {
    fn default() -> Self {
        Bounded((MIN + MAX) / 2.0)
    }
}

pub struct EntityPlugin;

impl EntityPlugin {
    fn initialize_materials_system(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
    ) {
        let damaged = materials.add(Color::rgb_u8(230, 18, 18).into());
        let gun = materials.add(Color::rgb_u8(204, 178, 153).into());
        commands.insert_resource(Materials { damaged, gun });
    }
    fn color_reset_system(mut commands: Commands, query: Query<(Entity, &EntityColor, &Unit)>) {
        for (e, color, _) in query.iter() {
            commands.insert_one(e, color.0.clone());
            commands.remove_one::<EntityColor>(e);
        }
    }
    fn fill_mesh_system(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        materials: Res<Materials>,
        query: Query<Without<Mesh, (Entity, &Stats, &Position, &EntityColor, &Unit)>>,
    ) {
        for (e, stats, position, color, _) in query.iter() {
            let size = 3.0 + stats.health.between();
            let gunsize = stats.firepower.between() * size;
            let child = commands
                .insert(
                    e,
                    PbrComponents {
                        material: color.0.clone(),
                        mesh: meshes.add(Mesh::from(Cube { size })),
                        transform: position.0,
                        ..Default::default()
                    },
                )
                .spawn(PbrComponents {
                    material: materials.gun.clone(),
                    mesh: meshes.add(Mesh::from(Cube { size: gunsize })),
                    transform: Transform::from_translation(Vec3::new(
                        0.0,
                        0.0,
                        -size + gunsize - stats.range.between() * 2.0 * gunsize,
                    )),
                    ..Default::default()
                })
                .current_entity();
            commands
                .push_children(e, &[child.unwrap()])
                .remove_one::<Position>(e);
        }
    }
    fn move_system(
        mut commands: Commands,
        time: Res<Time>,
        mut query: Query<(Entity, &Move, &Stats, &mut Transform, &Unit)>,
    ) {
        for (e, delta, stats, mut position, _) in query.iter_mut() {
            let delta = delta.delta;
            if delta.length_squared() > MOVEMENT_TOLERANCE {
                eprintln!("Movement value too large. Delta is {:?}, which has a magnitude greater than 1.", delta);
            } else {
                let delta = delta * stats.movement.0;
                *position =
                    Transform::from_translation(delta * time.delta.as_secs_f32()) * *position;
            }
            commands.remove_one::<Move>(e);
        }
    }
    fn find_target_system(
        mut commands: Commands,
        fire_query: Query<(Entity, &Fire, &Transform, &Stats, &Unit)>,
        target_query: Query<(&Transform, &Unit)>,
    ) {
        for (e, fire, position, stats, _) in fire_query.iter() {
            commands.remove_one::<Fire>(e);
            let (other_position, _) = target_query.get(fire.target).expect("Invalid Target");
            if (other_position.translation - position.translation).length()
                > FIRE_TOLERANCE * stats.range.0
            {
                panic!(
                    "Target at position {:?} is too far away for entity at position {:?} to fire at. Entity's stats are {:#?}",
                    (other_position).translation, position.translation, stats
                );
            } else {
                commands.insert_one(
                    e,
                    FireAt {
                        target: fire.target,
                        position: other_position.translation,
                    },
                );
            }
        }
    }
    fn fire_system(
        mut commands: Commands,
        time: Res<Time>,
        materials: Res<Materials>,
        mut query: Query<(Entity, &FireAt, &mut Transform, &Stats, &Unit)>,
        mut others: Query<(Entity, &mut Health, &Handle<StandardMaterial>, &Unit)>,
    ) {
        for (e, fire, mut position, stats, _) in query.iter_mut() {
            let target = fire.target;
            let firepower = stats.firepower;
            let (other_entity, mut other_health, other_color, _) =
                others.get_mut(target).expect("Invalid Target");
            position.look_at(fire.position, Vec3::unit_z());
            other_health.0 -= firepower.0 * time.delta.as_secs_f32();
            commands
                .remove_one::<FireAt>(e)
                .remove_one::<Handle<StandardMaterial>>(other_entity)
                .insert_one(other_entity, materials.damaged.clone())
                .insert_one(other_entity, EntityColor(other_color.clone()));
        }
    }
    fn death_system(mut commands: Commands, e: Entity, health: &Health, _: &Unit) {
        if health.0 < 0.0 {
            commands.despawn_recursive(e);
        }
    }
}
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(Self::initialize_materials_system.system())
            .add_system_to_stage(stage::PRE_UPDATE, Self::fill_mesh_system.system())
            .add_system(Self::color_reset_system.system())
            .add_system_to_stage(stage::POST_UPDATE, Self::find_target_system.system())
            .add_system_to_stage(stage::POST_UPDATE, Self::fire_system.system())
            .add_system_to_stage(stage::POST_UPDATE, Self::move_system.system())
            .add_system_to_stage(stage::POST_UPDATE, Self::death_system.system());
        // First fire then move so that AIs work out correctly.
    }
}
