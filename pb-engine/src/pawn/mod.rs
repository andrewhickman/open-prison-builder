use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;
use serde::{Deserialize, Serialize};

use crate::PIXELS_PER_METER;

pub const RADIUS: f32 = 0.16;

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Pawn;

#[derive(Default, Clone, Bundle)]
pub struct PawnBundle {
    pawn: Pawn,
    velocity: Velocity,
    transform: TransformBundle,
}

impl PawnBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            pawn: default(),
            velocity: Velocity::default(),
            transform: Transform::from_translation(position.extend(0.) * PIXELS_PER_METER).into(),
        }
    }
}
