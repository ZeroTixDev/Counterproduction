use bevy::prelude::*;
use derive_new::*;

#[derive(Bundle)]
pub struct Unit {
    pub health: Health,
    pub position: Position,
    pub stats: Stats,
}

impl Unit {
    pub fn new(p: Vec3, s: Stats) -> Unit {
        return Unit {
            health: Health(s.health.0),
            position: Position(p),
            stats: s,
        }
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
