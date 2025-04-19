use std::collections::VecDeque;

use avian2d::prelude::*;
use bevy::prelude::*;
use pb_util::callback::spawn_compute;
use tokio::sync::oneshot;
use vleue_navigator::prelude::*;

use crate::pawn::{ai::Task, Pawn, MAX_VELOCITY};

pub mod movement {
    include!(concat!(env!("OUT_DIR"), "/", "movement.rs"));
}

#[derive(Bundle)]
pub struct PathTaskBundle {
    task: Task,
    path: PathTask,
}

#[derive(Debug, Component)]
pub enum PathTask {
    Pending(oneshot::Receiver<Option<VecDeque<Vec2>>>),
    Running(VecDeque<Vec2>),
}

impl PathTaskBundle {
    pub fn path_to(actor: Entity, mesh: NavMesh, from: Vec2, to: Vec2) -> Self {
        let (tx, rx) = oneshot::channel();
        spawn_compute(async move {
            let res = if let Some(path) = mesh
                .get_transformed_path(from.extend(0.), to.extend(0.))
                .await
            {
                Some(path.path.into_iter().map(|step| step.xy()).collect())
            } else {
                None
            };

            let _ = tx.send(res);
        });

        PathTaskBundle {
            task: Task::new(actor),
            path: PathTask::Pending(rx),
        }
    }

    pub fn move_to(actor: Entity, to: Vec2) -> Self {
        PathTaskBundle {
            task: Task::new(actor),
            path: PathTask::Running(VecDeque::from_iter([to])),
        }
    }
}

pub fn update(
    mut commands: Commands,
    mut task_q: Query<(Entity, &Task, &mut PathTask)>,
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
) {
    for (id, task, mut path) in &mut task_q {
        let Ok((mut pawn, position, rotation, linear_velocity, angular_velocity, collisions)) =
            pawn_q.get_mut(task.actor)
        else {
            warn!("invalid target for PathTask");
            return;
        };

        if !collisions.is_empty() {
            info!("collided :(")
        }

        let Some(steps) = path.poll() else {
            return;
        };

        if let Some(&next_step) = steps.front() {
            let inv_isometry = Isometry2d::new(position.0, (*rotation).into()).inverse();

            let pawn_space_target = inv_isometry * next_step;
            let pawn_space_linear_velocity = inv_isometry.rotation * linear_velocity.0;

            let distance_remaining = pawn_space_target.length();
            if distance_remaining <= 0.1 {
                steps.pop_front();
            } else {
                let [[force_angle, torque, _, _]] = movement::main_graph([[
                    pawn_space_linear_velocity.to_angle(),
                    pawn_space_linear_velocity.length_squared() / (MAX_VELOCITY * MAX_VELOCITY),
                    angular_velocity.0,
                    pawn_space_target.to_angle(),
                    pawn_space_target.length().min(10.),
                ]]);

                pawn.update_movement(force_angle, 1., torque);
            }
        } else {
            pawn.dir = Vec2::ZERO;
            pawn.torque = 0.;
            commands.entity(id).despawn_recursive();
        }
    }
}

impl PathTask {
    fn poll(&mut self) -> Option<&mut VecDeque<Vec2>> {
        match self {
            PathTask::Running(steps) => Some(steps),
            PathTask::Pending(receiver) => {
                if let Ok(Some(steps)) = receiver.try_recv() {
                    *self = PathTask::Running(steps);
                    match self {
                        PathTask::Pending(_) => unreachable!(),
                        PathTask::Running(steps) => Some(steps),
                    }
                } else {
                    None
                }
            }
        }
    }
}
