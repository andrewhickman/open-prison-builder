pub mod task;

use std::f32::consts::{PI, TAU};

use avian2d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::layer::Layer;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(
    Name::new("Pawn"),
    RigidBody::Dynamic,
    Collider::circle(Pawn::RADIUS),
    CollisionLayers::new(Layer::Pawn, LayerMask::ALL),
    CollidingEntities,
    TranslationInterpolation,
    LinearDamping(0.5),
    AngularDamping(0.5)
)]
pub struct Pawn;

impl Pawn {
    pub const RADIUS: f32 = 0.16;
    pub const AREA: f32 = Self::RADIUS * Self::RADIUS * PI;
    pub const MAX_ACCELERATION: f32 = 0.68;
    pub const MAX_VELOCITY: f32 = 1.5;
    pub const REVERSE_VELOCITY: f32 = 0.7;
    pub const MAX_TORQUE: f32 = TAU;
    pub const MAX_ANGULAR_VELOCITY: f32 = PI;
    pub const VISION_RADIUS: f32 = 4.;

    pub fn bundle(position: Vec2, rotation: f32) -> (Pawn, Transform, Position, Rotation) {
        (
            Pawn,
            Transform::from_translation(position.extend(0.))
                .with_rotation(Quat::from_axis_angle(Vec3::Z, rotation)),
            Position(position),
            Rotation::radians(rotation),
        )
    }
}
