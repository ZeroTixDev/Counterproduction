use super::entities::*;
use super::AI;
use bevy::prelude::*;
use derive_new::*;

#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct Resources(pub f32);
#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct ResourceGain(pub f32);
#[derive(new, Clone, PartialEq, Default, Debug)]
pub struct PlayerColor(pub Handle<StandardMaterial>);
#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct PlayerColorUninitialized(pub Color);
/// The Parent is the Player in question.
#[derive(new, Clone, Copy, PartialEq, Debug)]
pub struct PlayerUnit(pub Stats, pub AI);
#[derive(Bundle)]
pub struct PlayerProps {
    position: Vec3,
    starting: Resources,
    gain: ResourceGain,
    color: PlayerColorUninitialized,
}
impl PlayerProps {
    fn player_stats() -> Stats {
        Stats {
            // Bounded override.
            health: Bounded(100.0),
            firepower: Bounded::min_b(),
            movement: Bounded::min_b(),
            range: Bounded::min_b(),
            reload: Bounded::min_b(),
            priority: -1.0,
            price: 0.0,
        }
    }

    pub fn new(position: Vec3, gain: f32, color: Color) -> Self {
        PlayerProps {
            position,
            starting: Resources(9999.0), // Enough resources for now.
            gain: ResourceGain(gain),
            color: PlayerColorUninitialized(color),
        }
    }
}
pub struct PlayerPlugin;
impl PlayerPlugin {
    fn initialize(
        mut commands: Commands,
        mut materials: ResMut<Assets<StandardMaterial>>,
        e: Entity,
        color: &PlayerColorUninitialized,
    ) {
        commands
            .insert_one(e, PlayerColor(materials.add(color.0.into())))
            .remove_one::<PlayerColorUninitialized>(e)
            .spawn((PlayerUnit(PlayerProps::player_stats(), AI::Nothing),))
            .with(Parent(e));
    }
    fn gain_resources(
        time: Res<Time>,
        mut resources: Mut<Resources>,
        resource_gain: &ResourceGain,
    ) {
        resources.0 += resource_gain.0 * time.delta.as_secs_f32();
    }
    fn spawn(
        mut commands: Commands,
        mut players: Query<(&PlayerColor, &mut Resources, &Vec3)>,
        unit: Query<(Entity, &Parent, &PlayerUnit)>,
    ) {
        for (e, Parent(player), PlayerUnit(stats, ai)) in unit.iter() {
            let (material, mut resources, position) =
                players.get_mut(*player).expect("Invalid Player");
            let price = stats.price;
            if price > resources.0 {
                eprintln!(
                    "Unit Price too large. Player {:?} has resources {:?}, which less than price {} for stats {:#?}",
                    player, resources, price, stats
                );
            } else {
                resources.0 -= price;
                commands
                    .spawn(UnitProps::new(
                        *position + Vec3::new(rand::random(), rand::random(), rand::random()) * 30.0,
                        *stats,
                        material.0.clone(),
                    ))
                    .with(*ai)
                    .with(PlayerControl(*player));
            }
            commands.despawn(e);
        }
    }
}
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(stage::PRE_UPDATE, Self::initialize.system())
            .add_system(Self::gain_resources.system())
            .add_system(Self::spawn.system());
    }
}
