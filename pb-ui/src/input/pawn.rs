use avian2d::prelude::*;
use bevy::{input::ButtonState, prelude::*};
use pb_engine::pawn::{Pawn, task::Task};

#[derive(Copy, Clone, Debug, Component)]
pub struct InputTask {
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
    Forward,
    Right,
    Backward,
}

pub fn input(trigger: Trigger<MovementInput>, mut task_q: Query<&mut InputTask>) {
    for mut task in &mut task_q {
        // TODO input smoothing?
        match trigger.dir {
            MovementDirection::Left => task.dir.y += delta(trigger.state),
            MovementDirection::Forward => task.dir.x += delta(trigger.state),
            MovementDirection::Right => task.dir.y -= delta(trigger.state),
            MovementDirection::Backward => task.dir.x -= delta(trigger.state),
        }
    }
}

pub fn update(task_q: Query<(&Task, &InputTask)>, mut pawn_q: Query<&mut LinearVelocity>) {
    for (task, input) in task_q {
        if let Ok(mut velocity) = pawn_q.get_mut(task.target()) {
            velocity.0 = input.dir * Pawn::MAX_VELOCITY;
        }
    }
}

fn delta(state: ButtonState) -> f32 {
    match state {
        ButtonState::Pressed => 1.0,
        ButtonState::Released => -1.0,
    }
}
