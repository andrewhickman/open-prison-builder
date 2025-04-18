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
        pawn.accel = 1.;
        match trigger.dir {
            MovementDirection::Left => pawn.dir.y += delta(trigger.state),
            MovementDirection::Forward => pawn.dir.x += delta(trigger.state),
            MovementDirection::Right => pawn.dir.y -= delta(trigger.state),
            MovementDirection::Backward => pawn.dir.x -= delta(trigger.state),
        }
    }
}

fn delta(state: ButtonState) -> f32 {
    match state {
        ButtonState::Pressed => 1.0,
        ButtonState::Released => -1.0,
    }
}
