use bevy::{input::ButtonState, prelude::*};
use pb_engine::pawn::Pawn;

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

pub fn input(trigger: Trigger<MovementInput>, mut pawn_q: Query<&mut Pawn>) {
    for mut pawn in &mut pawn_q {
        match trigger.dir {
            MovementDirection::Left => pawn.movement.y += delta(trigger.state),
            MovementDirection::Forward => pawn.movement.x += delta(trigger.state),
            MovementDirection::Right => pawn.movement.y -= delta(trigger.state),
            MovementDirection::Backward => pawn.movement.x -= delta(trigger.state),
        }
    }
}

fn delta(state: ButtonState) -> f32 {
    match state {
        ButtonState::Pressed => 1.0,
        ButtonState::Released => -1.0,
    }
}
