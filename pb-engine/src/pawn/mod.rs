pub mod ai;

use std::f32::consts::{PI, TAU};

use ai::Actor;
use approx::relative_ne;
use avian2d::prelude::*;
use bevy::prelude::*;
use pb_util::math::to_finite_f32_lossy;
use serde::{Deserialize, Serialize};

use crate::picking::Layer;

#[derive(Debug, Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(
    Actor,
    RigidBody::Dynamic,
    Collider::circle(Pawn::RADIUS),
    CollisionLayers::new(Layer::Pawn, LayerMask::ALL),
    CollidingEntities,
    TranslationInterpolation,
    LinearDamping(0.5),
    AngularDamping(0.5)
)]
pub struct Pawn {
    pub dir: Vec2,
    pub accel: f32,
    pub torque: f32,
}

#[derive(Default, Clone, Bundle)]
pub struct PawnBundle {
    pawn: Pawn,
    transform: Transform,
    position: Position,
    rotation: Rotation,
}

impl Pawn {
    pub const RADIUS: f32 = 0.16;
    pub const AREA: f32 = Self::RADIUS * Self::RADIUS * PI;
    pub const MAX_ACCELERATION: f32 = 0.68;
    pub const MAX_VELOCITY: f32 = 1.5;
    pub const REVERSE_VELOCITY: f32 = 0.7;
    pub const MAX_TORQUE: f32 = TAU;
    pub const MAX_ANGULAR_VELOCITY: f32 = PI;
    pub const VISION_RADIUS: f32 = 10.;

    pub fn update_movement(&mut self, angle: f32, accel: f32, torque: f32) {
        self.dir = Vec2::from_angle(to_finite_f32_lossy(angle).clamp(-1., 1.) * PI);
        self.accel = to_finite_f32_lossy(accel).clamp(0., 1.);
        self.torque = to_finite_f32_lossy(torque).clamp(-1., 1.);
    }
}

impl PawnBundle {
    pub fn new(position: Vec2, rotation: f32) -> Self {
        Self {
            pawn: default(),
            transform: Transform::from_translation(position.extend(0.))
                .with_rotation(Quat::from_axis_angle(Vec3::Z, rotation)),
            position: Position(position),
            rotation: Rotation::radians(rotation),
        }
    }
}

pub fn movement(
    mut pawn_q: Query<(
        &Pawn,
        &Rotation,
        &LinearVelocity,
        &AngularVelocity,
        &mut ExternalForce,
        &mut ExternalTorque,
    )>,
) {
    pawn_q.par_iter_mut().for_each(
        |(pawn, rotation, linear_velocity, angular_velocity, mut force, mut torque)| {
            force.persistent = false;
            torque.persistent = false;

            if relative_ne!(pawn.dir, Vec2::ZERO) {
                let movement_dir = rotation * pawn.dir;
                force.set_force(movement_dir.normalize() * pawn.accel * Pawn::MAX_ACCELERATION);
            } else if relative_ne!(linear_velocity.0, Vec2::ZERO) {
                force.set_force((-linear_velocity.0).normalize() * Pawn::MAX_ACCELERATION);
            }
            if relative_ne!(pawn.torque, 0.) {
                torque.apply_torque(pawn.torque * Pawn::MAX_TORQUE);
            } else if relative_ne!(angular_velocity.0, 0.) {
                torque.apply_torque((-angular_velocity.0).signum() * Pawn::MAX_TORQUE);
            }
        },
    );
}

pub fn clamp_velocity(
    mut pawn_q: Query<(&Rotation, &mut LinearVelocity, &mut AngularVelocity), With<Pawn>>,
) {
    pawn_q
        .par_iter_mut()
        .for_each(|(rotation, mut linear_velocity, mut angular_velocity)| {
            let mut velocity = linear_velocity.length();

            if relative_ne!(velocity, 0.0) {
                let forward_velocity = rotation.inverse() * linear_velocity.0;
                let angle_t = forward_velocity.to_angle().abs() / PI;
                let max_velocity = Pawn::MAX_VELOCITY.lerp(Pawn::MAX_VELOCITY / 2., angle_t);

                if velocity > max_velocity {
                    linear_velocity.0 *= max_velocity / velocity;
                    velocity = max_velocity;
                }
            }

            if relative_ne!(angular_velocity.0, 0.0) {
                let max_angular_velocity = Pawn::MAX_ANGULAR_VELOCITY.lerp(
                    Pawn::MAX_ANGULAR_VELOCITY / 2.,
                    velocity / Pawn::MAX_VELOCITY,
                );
                angular_velocity.0 =
                    angular_velocity.clamp(-max_angular_velocity, max_angular_velocity);
            }
        });
}
