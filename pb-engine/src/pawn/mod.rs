// pub mod ai;

use std::f32::consts::{PI, TAU};

use avian2d::prelude::*;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::picking::Layer;

pub const RADIUS: f32 = 0.16;
pub const MAX_ACCELERATION: f32 = 0.68;
pub const MAX_VELOCITY: f32 = 1.5;
pub const REVERSE_VELOCITY: f32 = 0.7;
pub const MAX_TORQUE: f32 = TAU;
pub const MAX_ANGULAR_VELOCITY: f32 = PI;

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(
    RigidBody(|| RigidBody::Dynamic),
    Collider(|| Collider::circle(RADIUS)),
    CollisionLayers(|| CollisionLayers::new(Layer::Pawn, LayerMask::ALL)),
    CollidingEntities,
    TranslationInterpolation,
    LinearDamping,
    AngularDamping,
)]
pub struct Pawn;

#[derive(Default, Clone, Bundle)]
pub struct PawnBundle {
    pawn: Pawn,
    transform: Transform,
    position: Position,
    rotation: Rotation,
}

impl PawnBundle {
    pub fn new(position: Vec2) -> Self {
        Self {
            pawn: default(),
            transform: Transform::from_translation(position.extend(0.)),
            position: Position(position),
            rotation: Rotation::default(),
        }
    }
}

pub fn clamp_velocity(
    mut pawn_q: Query<(&Rotation, &mut LinearVelocity, &mut AngularVelocity), With<Pawn>>,
) {
    for (rotation, mut linear_velocity, mut angular_velocity) in &mut pawn_q {
        let mut velocity = linear_velocity.length();

        if velocity > 0.0 {
            let forward_velocity = rotation.inverse() * linear_velocity.0;
            let angle_t = forward_velocity.to_angle().abs() / PI;
            let max_velocity = MAX_VELOCITY.lerp(MAX_VELOCITY / 2., angle_t);

            if velocity > max_velocity {
                linear_velocity.0 *= max_velocity / velocity;
                velocity = max_velocity;
            }
        }

        if angular_velocity.0 > 0.0 {
            let max_angular_velocity =
                MAX_ANGULAR_VELOCITY.lerp(MAX_ANGULAR_VELOCITY / 2., velocity / MAX_VELOCITY);
            angular_velocity.0 =
                angular_velocity.clamp(-max_angular_velocity, max_angular_velocity);
        }
    }
}
