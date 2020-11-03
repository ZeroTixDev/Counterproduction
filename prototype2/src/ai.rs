use super::entities::UnitProps;
use bevy::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AI {
    Nothing,
    Simple,
}

pub struct AIPlugin;

impl AI {
    fn move_step(self, this: UnitProps, all: impl Iterator<Item = UnitProps>) -> Vec3 {
        match self {
            AI::Nothing => todo!(),
            AI::Simple => todo!(),
        }
    }

    fn fire_step(self, this: UnitProps, all: impl Iterator<Item = (Entity, UnitProps)>) -> Entity {
        match self {
            AI::Nothing => todo!(),
            AI::Simple => todo!(),
        }
    }
}
impl AIPlugin {
    pub fn move_system() {}
    pub fn fire_system() {}
}
impl Plugin for AIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(Self::move_system.system())
            .add_system(Self::fire_system.system());
    }
}
