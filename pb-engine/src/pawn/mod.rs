use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;
use serde::{Deserialize, Serialize};

pub const RADIUS: f32 = 0.16;

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(Velocity)]
pub struct Pawn;

#[derive(Default, Clone, Bundle)]
pub struct PawnBundle {
    pawn: Pawn,
    transform: Transform,
}

impl PawnBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            pawn: default(),
            transform: Transform::from_translation(position.extend(0.)),
        }
    }
}
