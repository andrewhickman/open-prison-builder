use std::collections::VecDeque;

use avian2d::prelude::{CollidingEntities, LinearVelocity};
use bevy::prelude::*;
use pb_util::try_opt;
use tokio::sync::oneshot;
use vleue_navigator::prelude::*;

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
    result: Option<oneshot::Sender<PathTaskResult>>,
}

#[derive(Debug)]
enum PathTaskResult {
    Success,
    Collided { position: Vec2, navmesh: NavMesh },
}

pub fn update(
    mut commands: Commands,
    mut task_q: Query<(Entity, &mut PathTask)>,
    mut pawn_q: Query<(&Transform, &mut LinearVelocity, &CollidingEntities), With<Pawn>>,
    time: Res<Time<Virtual>>,
    navmesh_q: Option<Single<&ManagedNavMesh>>,
    navmeshes: Res<Assets<NavMesh>>,
) {
    if time.is_paused() {
        return;
    }

    let navmesh = try_opt!(navmeshes.get(try_opt!(navmesh_q).id()));

    for (id, mut task) in &mut task_q {
        let Ok((transform, velocity, collisions)) = &mut pawn_q.get_mut(task.target) else {
            warn!("invalid target for PathTask");
            continue;
        };

        if !collisions.is_empty() {
            if let Some(tx) = task.result.take() {
                let _ = tx.send(PathTaskResult::Collided {
                    position: transform.translation.xy(),
                    navmesh: navmesh.clone(),
                });
            }
            commands.entity(id).despawn_recursive();
        }

        if let Some(next_step) = task.steps.front() {
            let dir = *next_step - transform.translation.xy();
            let distance_remaining = dir.length();
            if distance_remaining <= time.delta_secs() * pawn::SPEED {
                velocity.0 = dir / time.delta_secs();
                task.steps.pop_front();
            } else {
                velocity.0 = dir / distance_remaining * pawn::SPEED;
            }
        } else {
            velocity.0 = Vec2::ZERO;
            if let Some(tx) = task.result.take() {
                let _ = tx.send(PathTaskResult::Success);
            }
            commands.entity(id).despawn_recursive();
        }
    }
}

impl PawnActor {
    pub async fn move_to(&self, mut mesh: NavMesh, mut from: Vec2, to: Vec2) -> TaskResult<()> {
        loop {
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
                    Ok(PathTaskResult::Success) => return TaskResult::Ok(()),
                    Ok(PathTaskResult::Collided { position, navmesh }) => {
                        warn!("collided, recalculating path");
                        from = position;
                        mesh = navmesh;
                        continue;
                    }
                    Err(_) => return TaskResult::Cancelled,
                }
            } else {
                return TaskResult::Err(format!("no path found from {from} to {to}").into());
            }
        }
    }
}
