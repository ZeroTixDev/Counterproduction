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
    &'a Parent,
    &'a Unit,
);

impl AI {
    #[allow(unused_variables)]
    fn move_step(self, this: UnitData, all: impl Iterator<Item = UnitData>) -> Option<Move> {
        match self {
            AI::Nothing => None,
            AI::Simple => Some(Move::new(Vec3::new(1.0, 0.0, 0.0))),
        }
    }

    #[allow(unused_variables)]
    fn fire_step(
        self,
        this: UnitData,
        mut all: impl Iterator<Item = (Entity, UnitData)>,
    ) -> Option<Fire> {
        match self {
            AI::Nothing => None,
            AI::Simple => all.next().map(|a| Fire::new(a.0)),
        }
    }
}
impl AIPlugin {
    fn move_system(mut commands: Commands, query: Query<UnitAIQuery>) {
        let map = |a: UnitAIQuery| {
            (
                *a.0,
                a.1,
                UnitData {
                    health: (a.2).0,
                    position: *a.3,
                    stats: *a.4,
                    parent: (a.5).0,
                },
            )
        };
        for (ai, e, data) in query.iter().map(map) {
            let step = AI::move_step(
                ai,
                data,
                query.iter().map(map).filter(|a| a.1 != e).map(|a| a.2),
            );
            if let Some(step) = step {
                commands.insert_one(e, step);
            }
        }
    }
    fn fire_system(mut commands: Commands, query: Query<UnitAIQuery>) {
        let map = |a: UnitAIQuery| {
            (
                *a.0,
                (
                    a.1,
                    UnitData {
                        health: (a.2).0,
                        position: *a.3,
                        stats: *a.4,
                        parent: (a.5).0,
                    },
                ),
            )
        };
        for (ai, (e, data)) in query.iter().map(map) {
            let step = AI::fire_step(
                ai,
                data,
                query.iter().map(map).filter(|a| (a.1).0 != e).map(|a| a.1),
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
