use std::collections::VecDeque;

use avian2d::prelude::*;
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

pub mod movement {
    include!(concat!(env!("OUT_DIR"), "/", "movement.rs"));
}

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
    mut pawn_q: Query<
        (
            &mut Pawn,
            &Position,
            &Rotation,
            &LinearVelocity,
            &AngularVelocity,
            &CollidingEntities,
        ),
        With<Pawn>,
    >,
    time: Res<Time>,
    navmesh_q: Option<Single<&ManagedNavMesh>>,
    navmeshes: Res<Assets<NavMesh>>,
) {
    let navmesh = try_opt!(navmeshes.get(try_opt!(navmesh_q).id()));

    for (id, mut task) in &mut task_q {
        let Ok((mut pawn, position, rotation, linear_velocity, angular_velocity, collisions)) =
            pawn_q.get_mut(task.target)
        else {
            warn!("invalid target for PathTask");
            continue;
        };

        if !collisions.is_empty() {
            if let Some(tx) = task.result.take() {
                let _ = tx.send(PathTaskResult::Collided {
                    position: position.0,
                    navmesh: navmesh.clone(),
                });
            }
            commands.entity(id).despawn_recursive();
        }

        if let Some(&next_step) = task.steps.front() {
            let inv_isometry = Isometry2d::new(position.0, (*rotation).into()).inverse();

            let pawn_space_target = inv_isometry * next_step;
            let pawn_space_linear_velocity = inv_isometry * linear_velocity.0;

            let distance_remaining = pawn_space_target.length();
            if distance_remaining <= 0.1 {
                task.steps.pop_front();
            } else {
                let [[force_x, force_y, _, _]] = movement::main_graph([[
                    pawn_space_linear_velocity.x,
                    pawn_space_linear_velocity.y,
                    angular_velocity.0,
                    pawn_space_target.x,
                    pawn_space_target.y,
                ]]);

                pawn.movement = Vec2::new(normalize(force_x), normalize(force_y));
            }
        } else {
            pawn.movement = Vec2::ZERO;
            if let Some(tx) = task.result.take() {
                let _ = tx.send(PathTaskResult::Success);
            }
            commands.entity(id).despawn_recursive();
        }
    }
}

fn normalize(f: f32) -> f32 {
    if f.is_finite() {
        f
    } else {
        0.
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
