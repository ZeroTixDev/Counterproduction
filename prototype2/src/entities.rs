use bevy::prelude::*;
use derive_new::*;
use shape::*;

const MOVEMENT_TOLERANCE: f32 = 1.01;
const FIRE_TOLERANCE: f32 = 1.01;

#[derive(Clone, Eq, PartialEq, Default, Debug)]
struct Materials {
    pub damaged: Handle<StandardMaterial>,
    pub gun: Handle<StandardMaterial>,
    pub targeting_area: Handle<StandardMaterial>,
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
    loading: LoadingStatus,
}

#[derive(Clone, PartialEq, Debug)]
pub struct UnitData {
    pub stats: Stats,
    pub position: Transform,
    pub health: f32,
    pub player: Entity,
}

impl UnitProps {
    pub fn new(position: Vec3, stats: Stats, material: Handle<StandardMaterial>) -> Self {
        UnitProps {
            position: Position(position),
            stats,
            unit: Unit,
            health: Health(stats.health.0),
            material: EntityColor(material),
            loading: LoadingStatus(0.0),
        }
    }
}

#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct Health(pub f32);
#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct LoadingStatus(pub f32);
#[derive(new, Clone, Copy, PartialEq, Debug)]
pub struct PlayerControl(pub Entity);
#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
struct Position(Vec3);
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

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Stats {
    pub health: Bounded<1.0, 10.0>,
    pub firepower: Bounded<0.5, 3.0>,
    pub range: Bounded<5.0, 50.0>,
    pub movement: Bounded<1.0, 3.0>,
    pub reload: Bounded<0.015, 7.0>,
    pub priority: f32,
    pub price: f32,
}

impl Stats {
    pub fn new(h: f32, f: f32, r: f32, m: f32, re: f32) -> Self {
        let mut stats = Stats {
            health: Bounded::new(h),
            firepower: Bounded::new(f),
            range: Bounded::new(r),
            movement: Bounded::new(m),
            reload: Bounded::new(re),
            priority: 0.0,
            price: 0.0,
        };
        stats.compute();
        stats
    }

    fn compute(&mut self) {
        self.price = self.movement.0 * 2.0
            + self.range.0.powf(1.2) * (self.firepower.0 + 1.0 / 2.0) * (self.health.0 / 2.0 + 1.0);
        self.priority = self.price;
    }
}

impl Default for Stats {
    fn default() -> Self {
        let mut s = Stats {
            health: Default::default(),
            firepower: Default::default(),
            range: Default::default(),
            movement: Default::default(),
            reload: Default::default(),
            priority: 0.0,
            price: 0.0,
        };
        s.compute();
        s
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
    pub fn min_b() -> Self {
        Bounded(MIN)
    }
    pub fn max_b() -> Self {
        Bounded(MAX)
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
        let targeting_area = materials.add(Color::rgba_u8(230, 18, 18, 100).into());
        commands.insert_resource(Materials {
            damaged,
            gun,
            targeting_area,
        });
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
        query: Query<Without<Handle<Mesh>, (Entity, &Stats, &Position, &EntityColor, &Unit)>>,
    ) {
        for (e, stats, position, color, _) in query.iter() {
            let size = 3.0 + stats.health.between();
            let gunsize = stats.firepower.between() * size;
            commands
                .insert(
                    e,
                    PbrComponents {
                        material: color.0.clone(),
                        mesh: meshes.add(Mesh::from(Cube { size })),
                        transform: Transform::from_translation(position.0),
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
                .with(Parent(e))
                .spawn(PbrComponents {
                    material: materials.targeting_area.clone(),
                    mesh: meshes.add(Mesh::from(Icosphere {
                        radius: stats.range.0,
                        subdivisions: 5,
                    })),
                    draw: Draw {
                        is_transparent: true,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .with(Parent(e));
            commands.remove_one::<Position>(e);
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
                let translation = position.translation;
                position.look_at(translation + delta, Vec3::unit_y());
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
                eprintln!(
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
        mut query: Query<(
            Entity,
            &FireAt,
            &mut Transform,
            &Stats,
            &mut LoadingStatus,
            &Unit,
        )>,
        mut others: Query<(Entity, &mut Health, &mut Handle<StandardMaterial>, &Unit)>,
    ) {
        for (e, fire, mut position, stats, mut loading, _) in query.iter_mut() {
            let target = fire.target;
            let firepower = stats.firepower;
            let (other_entity, mut other_health, mut other_color, _) =
                others.get_mut(target).expect("Invalid Target");
            position.look_at(fire.position, Vec3::unit_y());
            let t = time.time_since_startup().as_secs_f32();
            if t - loading.0 > stats.reload.0 {
                println!("Self: {:?}, Other: {:?}", e, target);
                println!("dP: {:?}", position.translation - fire.position);
                println!("Firing: {}", t - loading.0);
                println!("Health diff: {}", firepower.0 * stats.reload.0);
                other_health.0 -= firepower.0 * stats.reload.0;
                loading.0 = t;
                commands.insert_one(other_entity, EntityColor(other_color.clone()));
                *other_color = materials.damaged.clone();
            }
            commands.remove_one::<FireAt>(e);
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
