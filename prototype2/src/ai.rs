use super::entities::*;
use bevy::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AI {
    Nothing,
    Simple,
}

pub struct AIPlugin;

type UnitAIQuery<'a> = (
    &'a AI,
    Entity,
    &'a Health,
    &'a Transform,
    &'a Stats,
    &'a PlayerControl,
    &'a Unit,
);

impl AI {
    #[allow(unused_variables)]
    fn move_step(
        self,
        this: UnitData,
        all: impl Iterator<Item = UnitData>,
        players: &Query<(Entity, &Vec3)>,
    ) -> Option<Move> {
        match self {
            AI::Nothing => None,
            AI::Simple => players
                .iter()
                .find(|x| x.0 != this.player)
                .map(|x| Move::new((*x.1 - this.position.translation).normalize())),
        }
    }

    #[allow(unused_variables)]
    fn fire_step(
        self,
        this: UnitData,
        all: impl Iterator<Item = (Entity, UnitData)>,
        players: &Query<(Entity, &Vec3)>,
    ) -> Option<Fire> {
        match self {
            AI::Nothing => None,
            AI::Simple => all
                .filter(|x| {
                    x.1.player != this.player
                        && (x.1.position.translation - this.position.translation).length()
                            <= this.stats.range.0
                })
                .fold_first(|a, b| {
                    if a.1.stats.priority > b.1.stats.priority {
                        a
                    } else {
                        b
                    }
                })
                .map(|x| Fire::new(x.0)),
        }
    }
}
impl AIPlugin {
    fn move_system(
        mut commands: Commands,
        query: Query<UnitAIQuery>,
        players: Query<(Entity, &Vec3)>,
    ) {
        let map = |a: UnitAIQuery| {
            (
                *a.0,
                a.1,
                UnitData {
                    health: (a.2).0,
                    position: *a.3,
                    stats: *a.4,
                    player: (a.5).0,
                },
            )
        };
        for (ai, e, data) in query.iter().map(map) {
            let step = AI::move_step(ai, data, query.iter().map(|a| map(a).2), &players);
            if let Some(step) = step {
                commands.insert_one(e, step);
            }
        }
    }
    fn fire_system(
        mut commands: Commands,
        query: Query<UnitAIQuery>,
        players: Query<(Entity, &Vec3)>,
    ) {
        let map = |a: UnitAIQuery| {
            (
                *a.0,
                (
                    a.1,
                    UnitData {
                        health: (a.2).0,
                        position: *a.3,
                        stats: *a.4,
                        player: (a.5).0,
                    },
                ),
            )
        };
        for (ai, (e, data)) in query.iter().map(map) {
            let step = AI::fire_step(
                ai,
                data,
                query.iter().map(map).filter(|a| (a.1).0 != e).map(|a| a.1),
                &players,
            );
            if let Some(step) = step {
                commands.insert_one(e, step);
            }
        }
    }
}
impl Plugin for AIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::move_system.system())
            .add_system(Self::fire_system.system());
    }
}
