use approx::{relative_eq, relative_ne};
use avian2d::prelude::{AngularVelocity, ExternalForce, ExternalTorque, LinearVelocity, Rotation};
use bevy::{input::ButtonState, prelude::*};
use pb_engine::pawn::{MAX_ACCELERATION, MAX_TORQUE};

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct MovementState {
    dir: Vec2,
}

#[derive(Event, Debug, Clone, Copy)]
pub struct MovementInput {
    pub dir: MovementDirection,
    pub state: ButtonState,
}

#[derive(Debug, Clone, Copy)]
pub enum MovementDirection {
    Left,
    Up,
    Right,
    Down,
}

pub fn input(trigger: Trigger<MovementInput>, mut pawn_q: Query<&mut MovementState>) {
    for mut state in &mut pawn_q {
        match trigger.dir {
            MovementDirection::Left => state.dir -= Vec2::X * delta(trigger.state),
            MovementDirection::Up => state.dir += Vec2::Y * delta(trigger.state),
            MovementDirection::Right => state.dir += Vec2::X * delta(trigger.state),
            MovementDirection::Down => state.dir -= Vec2::Y * delta(trigger.state),
        }
    }
}

pub fn update(
    mut pawn_q: Query<(
        &MovementState,
        &Rotation,
        &LinearVelocity,
        &AngularVelocity,
        &mut ExternalForce,
        &mut ExternalTorque,
    )>,
) {
    for (state, rotation, linear_velocity, angular_velocity, mut force, mut torque) in &mut pawn_q {
        force.persistent = false;
        torque.persistent = false;

        if relative_eq!(state.dir, Vec2::ZERO) {
            if relative_ne!(linear_velocity.0, Vec2::ZERO) {
                force.apply_force((-linear_velocity.0).normalize() * MAX_ACCELERATION);
            }
            if relative_ne!(angular_velocity.0, 0.) {
                torque.apply_torque((-angular_velocity.0).signum() * MAX_TORQUE);
            }
        } else {
            force.apply_force(state.dir.normalize() * MAX_ACCELERATION);

            let angle = state.dir.angle_to(Vec2::new(-rotation.sin, rotation.cos));
            torque.apply_torque(-angle.signum() * MAX_TORQUE);
        }
    }
}

fn delta(state: ButtonState) -> f32 {
    match state {
        ButtonState::Pressed => 1.0,
        ButtonState::Released => -1.0,
    }
}
