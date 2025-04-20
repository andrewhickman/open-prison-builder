#[rustfmt::skip]
mod model;

use std::{collections::VecDeque, f32::consts::PI};

use avian2d::prelude::*;
use bevy::prelude::*;
use pb_util::callback::spawn_compute;
use tokio::sync::oneshot;
use vleue_navigator::prelude::*;

use crate::{
    pawn::{ai::Task, Pawn, MAX_ANGULAR_VELOCITY, MAX_VELOCITY, VISION_RADIUS},
    wall::Wall,
};

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

#[derive(Debug)]
pub struct PathObservation {
    pub linear_velocity_t: f32,
    pub linear_velocity_r: f32,
    pub angular_velocity: f32,
    pub target_t: f32,
    pub target_r: f32,
    pub collision_t: f32,
    pub collision_r: f32,
    pub collision_normal_t: f32,
    pub collision_is_wall: f32,
    pub collision_is_pawn: f32,
}

impl PathTaskBundle {
    pub fn path_to(actor: Entity, mesh: NavMesh, from: Vec2, to: Vec2) -> Self {
        let (tx, rx) = oneshot::channel();
        spawn_compute(async move {
            let res = if let Some(path) = mesh
                .get_transformed_path(from.extend(0.), to.extend(0.))
                .await
            {
                info!("found path: {path:?}");
                Some(path.path.into_iter().map(|step| step.xy()).collect())
            } else {
                info!("path not found");
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
            &ShapeHits,
        ),
        With<Pawn>,
    >,
    has_pawn_q: Query<(), With<Pawn>>,
    has_wall_q: Query<(), With<Wall>>,
    time: Res<Time>,
) {
    for (id, task, mut path) in &mut task_q {
        let Ok((mut pawn, position, rotation, linear_velocity, angular_velocity, collisions)) =
            pawn_q.get_mut(task.actor)
        else {
            warn!("invalid target for PathTask");
            return;
        };

        let Some(steps) = path.poll() else {
            return;
        };

        if let Some(&next_step) = steps.front() {
            let obs = PathObservation::new(
                position,
                rotation,
                linear_velocity,
                angular_velocity,
                collisions,
                next_step,
                |id| has_pawn_q.contains(id),
                |id| has_wall_q.contains(id),
            );

            if obs.done(time.delta_secs()) {
                steps.pop_front();
            } else {
                let [[angle, force, torque, _, _, _]] = model::main_graph([obs.into()]);

                pawn.update_movement(angle, force, torque);
            }
        } else {
            pawn.dir = Vec2::ZERO;
            pawn.torque = 0.;
            commands.entity(id).despawn_recursive();
        }
    }
}

impl PathObservation {
    pub const SIZE: usize = 10;

    pub fn new(
        position: &Position,
        rotation: &Rotation,
        linear_velocity: &LinearVelocity,
        angular_velocity: &AngularVelocity,
        collisions: &ShapeHits,
        target: Vec2,
        mut is_pawn_fn: impl FnMut(Entity) -> bool,
        mut is_wall_fn: impl FnMut(Entity) -> bool,
    ) -> Self {
        let inv_isometry = Isometry2d::new(position.0, (*rotation).into()).inverse();

        let pawn_space_target = inv_isometry * target;
        let pawn_space_linear_velocity = inv_isometry.rotation * linear_velocity.0;

        let (collision_t, collision_r, collision_normal_t, collision_is_wall, collision_is_pawn) =
            if collisions.is_empty() {
                (PI, VISION_RADIUS, 0., 0., 0.)
            } else {
                let collision = &collisions.as_slice()[0];

                let pawn_space_point = inv_isometry * collision.point1;
                let pawn_space_normal = inv_isometry.rotation * collision.normal1;

                let is_pawn = is_pawn_fn(collision.entity);
                let is_wall = is_wall_fn(collision.entity);

                (
                    pawn_space_point.to_angle(),
                    collision.distance,
                    pawn_space_normal.to_angle(),
                    is_pawn as u32 as f32,
                    is_wall as u32 as f32,
                )
            };

        PathObservation {
            linear_velocity_t: pawn_space_linear_velocity.to_angle(),
            linear_velocity_r: pawn_space_linear_velocity.length_squared()
                / (MAX_VELOCITY * MAX_VELOCITY),
            angular_velocity: angular_velocity.0 / MAX_ANGULAR_VELOCITY,
            target_t: pawn_space_target.to_angle(),
            target_r: pawn_space_target.length().min(VISION_RADIUS),
            collision_t,
            collision_r,
            collision_normal_t,
            collision_is_wall,
            collision_is_pawn,
        }
    }

    pub fn done(&self, delta_secs: f32) -> bool {
        self.target_r < (MAX_VELOCITY * delta_secs)
    }
}

impl From<PathObservation> for [f32; PathObservation::SIZE] {
    fn from(obs: PathObservation) -> [f32; PathObservation::SIZE] {
        [
            obs.linear_velocity_t,
            obs.linear_velocity_r,
            obs.angular_velocity,
            obs.target_t,
            obs.target_r,
            obs.collision_t,
            obs.collision_r,
            obs.collision_normal_t,
            obs.collision_is_wall,
            obs.collision_is_pawn,
        ]
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
