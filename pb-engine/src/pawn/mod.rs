use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::PIXELS_PER_METER;

pub const RADIUS: f32 = 0.16;

#[derive(Default, Component)]
pub struct Pawn;

#[derive(Bundle)]
pub struct PawnBundle {
    pawn: Pawn,
    collider: Collider,
    transform: TransformBundle,
}

impl PawnBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            pawn: Default::default(),
            collider: Collider::ball(RADIUS * PIXELS_PER_METER),
            transform: Transform::from_translation(position.extend(0.) * PIXELS_PER_METER).into(),
        }
    }
}
