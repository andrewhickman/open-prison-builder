use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::PIXELS_PER_METER;

pub const RADIUS: f32 = 0.16;

#[derive(Default, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
pub struct Pawn;

#[derive(Bundle)]
pub struct PawnBundle {
    pawn: Pawn,
    transform: TransformBundle,
}

impl PawnBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            pawn: default(),
            transform: Transform::from_translation(position.extend(0.) * PIXELS_PER_METER).into(),
        }
    }
}
