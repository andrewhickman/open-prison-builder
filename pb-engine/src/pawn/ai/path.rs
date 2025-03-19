use std::collections::VecDeque;

use bevy::prelude::*;
use tokio::sync::oneshot;
use vleue_navigator::NavMesh;

use crate::pawn::{
    self,
    ai::{PawnActor, Task},
    Pawn,
};

use super::TaskResult;

#[derive(Debug, Component)]
#[require(Task)]
pub struct PathTask {
    target: Entity,
    steps: VecDeque<Vec2>,
    result: Option<oneshot::Sender<()>>,
}

pub fn update(
    mut commands: Commands,
    mut task_q: Query<(Entity, &mut PathTask)>,
    mut pawn_q: Query<&mut Transform, With<Pawn>>,
    time: Res<Time<Virtual>>,
) {
    if time.is_paused() {
        return;
    }

    for (id, mut task) in &mut task_q {
        let Ok(transform) = &mut pawn_q.get_mut(task.target) else {
            continue;
        };

        if let Some(next_step) = task.steps.front() {
            let distance_moved = time.delta_secs() * pawn::SPEED;

            let dir = *next_step - transform.translation.xy();
            let distance_remaining = dir.length();

            if distance_remaining <= distance_moved {
                transform.translation = next_step.extend(0.);
                task.steps.pop_front();
            } else {
                transform.translation += ((dir / distance_remaining) * distance_moved).extend(0.);
            }
        } else {
            if let Some(tx) = task.result.take() {
                let _ = tx.send(());
            }
            commands.entity(id).despawn_recursive();
        }
    }
}

impl PawnActor {
    pub async fn move_to(&self, mesh: NavMesh, from: Vec2, to: Vec2) -> TaskResult<()> {
        if let Some(path) = mesh
            .get_transformed_path(from.extend(0.), to.extend(0.))
            .await
        {
            let (tx, rx) = oneshot::channel();
            self.spawn_task(PathTask {
                target: self.target,
                steps: path.path.into_iter().map(|step| step.xy()).collect(),
                result: Some(tx),
            });

            match rx.await {
                Ok(()) => TaskResult::Ok(()),
                Err(_) => TaskResult::Cancelled,
            }
        } else {
            TaskResult::Err(format!("no path found from {from} to {to}").into())
        }
    }
}
