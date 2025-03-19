use bevy::prelude::*;

use crate::pawn::{self, ai::Task, Pawn};

#[derive(Debug, Component)]
#[require(Task)]
pub struct PathTask {
    target: Entity,
    steps: Vec<Vec2>,
}

pub fn update(
    mut commands: Commands,
    mut task_q: Query<(Entity, &mut PathTask)>,
    mut pawn_q: Query<&mut Transform, With<Pawn>>,
    time: Time<Virtual>,
) {
    if time.is_paused() {
        return;
    }

    for (id, mut task) in &mut task_q {
        let Ok(transform) = &mut pawn_q.get_mut(task.target) else {
            continue;
        };

        if let Some(next_step) = task.steps.first_mut() {
            let distance_moved = time.delta_secs() * pawn::SPEED;

            let dir = *next_step - transform.translation.xy();
            let distance_remaining = dir.length();

            if distance_remaining <= distance_moved {
                transform.translation = next_step.extend(0.);
                task.steps.pop();
            } else {
                transform.translation -= ((dir / distance_remaining) * distance_moved).extend(0.);
            }
        } else {
            commands.entity(id).despawn_recursive();
        }
    }
}
