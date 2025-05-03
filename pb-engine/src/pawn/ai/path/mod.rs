#[rustfmt::skip]
mod model;

use std::{collections::VecDeque, f32::consts::PI};

use avian2d::{collision::collider::contact_query, prelude::*};
use bevy::{
    ecs::{query::QueryEntityError, system::SystemParam},
    prelude::*,
};
use tokio::sync::oneshot;

use crate::{
    pawn::{MAX_ANGULAR_VELOCITY, MAX_VELOCITY, Pawn, VISION_RADIUS, ai::Task},
    picking::Layer,
    wall::Wall,
};

const POSITION_EPSILON: f32 = MAX_VELOCITY / 64.;

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

#[derive(SystemParam)]
pub struct PathQuery<'w, 's> {
    spatial_query: SpatialQuery<'w, 's>,
    pawn_q: Query<
        'w,
        's,
        (
            &'static mut Pawn,
            &'static Position,
            &'static Rotation,
            &'static Collider,
            &'static LinearVelocity,
            &'static AngularVelocity,
        ),
        With<Pawn>,
    >,
    collider_q: Query<
        'w,
        's,
        (
            &'static Position,
            &'static Rotation,
            &'static Collider,
            &'static LinearVelocity,
            Has<Wall>,
            Has<Pawn>,
        ),
    >,
    config: Res<'w, PathQueryConfig>,
}

#[derive(Resource)]
pub struct PathQueryConfig {
    collider: Collider,
    all_filter: SpatialQueryFilter,
    wall_filter: SpatialQueryFilter,
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
    pub collision_velocity_t: f32,
    pub collision_velocity_r: f32,
}

impl PathTaskBundle {
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
    mut path_q: PathQuery,
) {
    for (id, task, mut path) in &mut task_q {
        let Some(steps) = path.poll() else {
            return;
        };

        if steps.is_empty() {
            info!("completed path");
            path_q.act(task.actor, 0., 0., 0.).expect("invalid entity");
            commands.entity(id).despawn();
            return;
        }

        let obs = path_q.observe(task.actor, steps).expect("invalid entity");

        let [[angle, force, torque, _, _, _]] = model::main_graph([obs.into()]);
        path_q
            .act(task.actor, angle, force, torque)
            .expect("invalid entity");
    }
}

impl PathQuery<'_, '_> {
    pub fn observe(
        &self,
        entity: Entity,
        steps: &mut VecDeque<Vec2>,
    ) -> Result<PathObservation, QueryEntityError> {
        let (_, position, rotation, collider, linear_velocity, angular_velocity) =
            self.pawn_q.get(entity)?;

        let target = loop {
            let Some(&current_step) = steps.front() else {
                break None;
            };

            if position.distance_squared(current_step) < (POSITION_EPSILON * POSITION_EPSILON) {
                steps.pop_front();
                continue;
            }

            if let Some(&next_step) = steps.get(1) {
                if self.visible(position.0, next_step) {
                    steps.pop_front();
                    continue;
                }
            }

            break Some(current_step);
        };

        let collision = self.collision(entity, *position, *rotation, collider);

        Ok(PathObservation::new(
            position,
            rotation,
            linear_velocity,
            angular_velocity,
            collision,
            target.unwrap_or(position.0),
        ))
    }

    pub fn act(
        &mut self,
        entity: Entity,
        angle: f32,
        force: f32,
        torque: f32,
    ) -> Result<(), QueryEntityError> {
        let (mut pawn, _, _, _, _, _) = self.pawn_q.get_mut(entity)?;
        pawn.update_movement(angle, force, torque);
        Ok(())
    }

    fn collision(
        &self,
        entity: Entity,
        pawn_position: Position,
        pawn_rotation: Rotation,
        pawn_collider: &Collider,
    ) -> Option<PathCollision> {
        let inv_isometry = Isometry2d::new(pawn_position.0, pawn_rotation.into()).inverse();

        let mut result = None;
        self.spatial_query.shape_intersections_callback(
            &self.config.collider,
            pawn_position.0,
            pawn_rotation.as_radians(),
            &self.config.all_filter,
            |collider_entity| {
                if collider_entity == entity {
                    return true;
                }

                let Ok((
                    collider_position,
                    collider_rotation,
                    collider_shape,
                    collider_velocity,
                    collider_is_wall,
                    collider_is_pawn,
                )) = self.collider_q.get(collider_entity)
                else {
                    warn!("invalid collision entity");
                    return true;
                };

                if let Some(contact) = contact_query::contact(
                    pawn_collider,
                    pawn_position,
                    pawn_rotation,
                    collider_shape,
                    *collider_position,
                    *collider_rotation,
                    VISION_RADIUS,
                )
                .unwrap()
                {
                    let point2 = collider_rotation * contact.local_point2 - pawn_position.0;
                    let normal = inv_isometry.rotation * contact.global_normal2(collider_rotation);

                    let pawn_space_velocity = inv_isometry.rotation * collider_velocity.0;

                    let contact = PathCollision {
                        angle: point2.to_angle(),
                        distance: -contact.penetration,
                        normal: normal.to_angle(),
                        velocity_t: pawn_space_velocity.to_angle(),
                        velocity_r: pawn_space_velocity.length_squared()
                            / (MAX_VELOCITY * MAX_VELOCITY),
                        is_pawn: collider_is_pawn,
                        is_wall: collider_is_wall,
                    };

                    match &result {
                        None => result = Some(contact),
                        Some(closest_contact) if contact.distance < closest_contact.distance => {
                            result = Some(contact)
                        }
                        _ => (),
                    }
                }

                true
            },
        );

        result
    }

    fn visible(&self, position: Vec2, target: Vec2) -> bool {
        let delta = target - position;
        let Ok(dir) = Dir2::new(delta) else {
            return true;
        };

        self.spatial_query
            .cast_ray(
                position,
                dir,
                delta.length(),
                true,
                &self.config.wall_filter,
            )
            .is_none()
    }
}

impl Default for PathQueryConfig {
    fn default() -> Self {
        Self {
            collider: Collider::circle(VISION_RADIUS),
            all_filter: SpatialQueryFilter::DEFAULT,
            wall_filter: SpatialQueryFilter {
                mask: Layer::Wall.into(),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug)]
pub struct PathCollision {
    angle: f32,
    distance: f32,
    normal: f32,
    velocity_t: f32,
    velocity_r: f32,
    is_pawn: bool,
    is_wall: bool,
}

impl Default for PathCollision {
    fn default() -> Self {
        Self {
            angle: 1.,
            distance: VISION_RADIUS,
            normal: 0.,
            velocity_t: 0.,
            velocity_r: 0.,
            is_pawn: false,
            is_wall: false,
        }
    }
}

impl PathObservation {
    pub const SIZE: usize = 12;

    pub fn new(
        position: &Position,
        rotation: &Rotation,
        linear_velocity: &LinearVelocity,
        angular_velocity: &AngularVelocity,
        collision: Option<PathCollision>,
        target: Vec2,
    ) -> Self {
        let inv_isometry = Isometry2d::new(position.0, (*rotation).into()).inverse();

        let pawn_space_target = inv_isometry * target;
        let pawn_space_linear_velocity = inv_isometry.rotation * linear_velocity.0;

        let collision = collision.unwrap_or_default();

        PathObservation {
            linear_velocity_t: pawn_space_linear_velocity.to_angle() / PI,
            linear_velocity_r: pawn_space_linear_velocity.length_squared()
                / (MAX_VELOCITY * MAX_VELOCITY),
            angular_velocity: angular_velocity.0 / MAX_ANGULAR_VELOCITY,
            target_t: pawn_space_target.to_angle() / PI,
            target_r: pawn_space_target.length().min(VISION_RADIUS),
            collision_t: collision.angle / PI,
            collision_r: collision.distance,
            collision_normal_t: collision.normal / PI,
            collision_velocity_t: collision.velocity_t / PI,
            collision_velocity_r: collision.velocity_r,
            collision_is_wall: collision.is_wall as u32 as f32,
            collision_is_pawn: collision.is_pawn as u32 as f32,
        }
    }

    pub fn velocity_reward(&self) -> f32 {
        ((self.linear_velocity_t - self.target_t) * PI).cos() * self.linear_velocity_r
    }

    pub fn rotation_penalty(&self) -> f32 {
        -self.target_t.abs()
    }

    pub fn collision_penalty(&self) -> f32 {
        if self.collision_r < 0.1 {
            10. * self.collision_r - 1.
        } else {
            0.
        }
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
            obs.collision_velocity_t,
            obs.collision_velocity_r,
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

    #[cfg(feature = "dev")]
    fn steps(&self) -> Option<&VecDeque<Vec2>> {
        match self {
            PathTask::Pending(_) => None,
            PathTask::Running(steps) => Some(steps),
        }
    }
}

#[cfg(feature = "dev")]
pub fn debug_draw_path(
    task_q: Query<(&Task, &PathTask)>,
    pos_q: Query<&Position>,
    mut gizmos: Gizmos,
) {
    for (task, path) in &task_q {
        if let Some(steps) = path.steps() {
            if let Ok(start) = pos_q.get(task.actor) {
                if !steps.is_empty() {
                    gizmos.line_2d(
                        start.0,
                        steps[0],
                        bevy::color::palettes::tailwind::INDIGO_800,
                    );
                    for i in 0..(steps.len() - 1) {
                        gizmos.line_2d(
                            steps[i],
                            steps[i + 1],
                            bevy::color::palettes::tailwind::INDIGO_800,
                        );
                    }
                }
            }
        }
    }
}
