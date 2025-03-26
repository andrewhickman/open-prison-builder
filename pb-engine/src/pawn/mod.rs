pub mod ai;

use avian2d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::picking::Layer;

pub const RADIUS: f32 = 0.16;
pub const SPEED: f32 = 1.42;

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::circle(RADIUS)),
    CollisionLayers(|| CollisionLayers::new(Layer::Pawn, LayerMask::ALL)),
    CollidingEntities,
    TranslationInterpolation
)]
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
