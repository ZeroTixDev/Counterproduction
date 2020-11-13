use bevy::prelude::*;
use derive_new::*;

// A rather lame file, right?
#[derive(new, Clone, Copy, PartialEq, Default, Debug)]
pub struct Objective(pub Vec3);
