#[rustfmt::skip]
mod model;

use std::{collections::VecDeque, f32::consts::PI};

use avian2d::prelude::*;
use bevy::{
    ecs::{query::QueryEntityError, system::SystemParam},
    prelude::*,
};
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

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct PathTarget {
    pub position: Option<Vec2>,
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
            &'static mut PathTarget,
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
    filter: SpatialQueryFilter,
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
    mut path_q: PathQuery,
    time: Res<Time>,
) {
    for (id, task, mut path) in &mut task_q {
        let Some(steps) = path.poll() else {
            return;
        };

        loop {
            let obs = path_q.observe(task.actor).expect("invalid entity");

            info!("obs: {:#?}", obs);
            info!("velocity_reward: {}", obs.velocity_reward());
            info!(
                "angular_velocity_penalty: {}",
                obs.angular_velocity_penalty()
            );
            info!("rotation_penalty: {}", obs.rotation_penalty());
            info!("collision_penalty: {}", obs.collision_penalty());

            if obs.done(time.delta_secs()) {
                match steps.pop_front() {
                    Some(target) => {
                        path_q
                            .set_target(task.actor, Some(target))
                            .expect("invalid entity");
                    }
                    None => {
                        info!("completed path");
                        path_q.act(task.actor, 0., 0., 0.).expect("invalid entity");
                        path_q.set_target(task.actor, None).expect("invalid entity");
                        commands.entity(id).despawn_recursive();
                        break;
                    }
                }
            } else {
                let [[angle, force, torque, _, _, _]] = model::main_graph([obs.into()]);

                path_q
                    .act(task.actor, angle, force, torque)
                    .expect("invalid entity");
                break;
            }
        }
    }
}

// TODO onremove clear target

impl PathQuery<'_, '_> {
    pub fn observe(&self, entity: Entity) -> Result<PathObservation, QueryEntityError> {
        let (_, position, rotation, collider, linear_velocity, angular_velocity, target) =
            self.pawn_q.get(entity)?;

        let collision = self.collision(entity, *position, *rotation, collider);

        Ok(PathObservation::new(
            position,
            rotation,
            linear_velocity,
            angular_velocity,
            collision,
            target.position.unwrap_or(position.0),
        ))
    }

    pub fn set_target(
        &mut self,
        entity: Entity,
        position: Option<Vec2>,
    ) -> Result<(), QueryEntityError> {
        let (_, _, _, _, _, _, mut target) = self.pawn_q.get_mut(entity)?;
        target.position = position;
        Ok(())
    }

    pub fn act(
        &mut self,
        entity: Entity,
        angle: f32,
        force: f32,
        torque: f32,
    ) -> Result<(), QueryEntityError> {
        let (mut pawn, _, _, _, _, _, _) = self.pawn_q.get_mut(entity)?;
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
            &self.config.filter,
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
                    let point2 = collider_rotation * contact.point2 - pawn_position.0;
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
}

impl Default for PathQueryConfig {
    fn default() -> Self {
        Self {
            collider: Collider::circle(VISION_RADIUS),
            filter: SpatialQueryFilter::DEFAULT,
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

    pub fn done(&self, delta_secs: f32) -> bool {
        self.target_r < (MAX_VELOCITY * delta_secs)
    }

    pub fn velocity_reward(&self) -> f32 {
        ((self.linear_velocity_t - self.target_t) * PI).cos() * self.linear_velocity_r
    }

    pub fn rotation_penalty(&self) -> f32 {
        -self.target_t.abs()
    }

    pub fn angular_velocity_penalty(&self) -> f32 {
        -self.angular_velocity.abs()
    }

    pub fn collision_penalty(&self) -> f32 {
        -(-self.collision_r * 16.).exp2()
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
}
