use super::entities::*;
use bevy::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AI {
    Nothing,
    Simple,
}

pub struct AIPlugin;

impl AI {
    fn move_step(self, this: UnitData, all: impl Iterator<Item = UnitData>) -> Move {
        match self {
            AI::Nothing => todo!(),
            AI::Simple => todo!(),
        }
    }

    fn fire_step(self, this: UnitData, all: impl Iterator<Item = (Entity, UnitData)>) -> Fire {
        match self {
            AI::Nothing => todo!(),
            AI::Simple => todo!(),
        }
    }
}
impl AIPlugin {
    fn move_system(query: Query<(&AI, &Health, &Transform, &Stats, &Unit)>) {
        let map = |a: (&AI, &Health, &Transform, &Stats, &Unit)| {
            (
                *a.0,
                UnitData {
                    health: (*a.1).0,
                    position: *a.2,
                    stats: *a.3,
                },
            )
        };
        for (ai, data) in query.iter().map(map) {
            AI::move_step(ai, data, query.iter().map(map).map(|a| a.1));
        }
    }
    fn fire_system(query: Query<(&AI, Entity, &Health, &Transform, &Stats, &Unit)>) {
        let map = |a: (&AI, Entity, &Health, &Transform, &Stats, &Unit)| {
            (
                *a.0,
                (
                    a.1,
                    UnitData {
                        health: (*a.2).0,
                        position: *a.3,
                        stats: *a.4,
                    },
                ),
            )
        };
        for (ai, (_, data)) in query.iter().map(map) {
            AI::fire_step(ai, data, query.iter().map(map).map(|a| a.1));
        }
    }
}
impl Plugin for AIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::move_system.system())
            .add_system(Self::fire_system.system());
    }
}
