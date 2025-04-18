pub mod path;

use bevy::prelude::*;
use pb_util::try_res_s;

#[derive(Clone, Copy, Debug, Component)]
pub struct Task {
    actor: Entity,
}

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct Actor {
    task: Option<Entity>,
}

impl Task {
    fn new(actor: Entity) -> Self {
        Task { actor }
    }
}

pub fn task_added(
    trigger: Trigger<OnInsert, Task>,
    mut commands: Commands,
    task_q: Query<&Task>,
    mut actor_q: Query<&mut Actor>,
) {
    let task = try_res_s!(task_q.get(trigger.entity()));
    let mut actor = try_res_s!(actor_q.get_mut(task.actor));
    if let Some(prev_task) = actor.task.replace(trigger.entity()) {
        commands.entity(prev_task).try_despawn_recursive();
    }
}

pub fn task_removed(
    trigger: Trigger<OnReplace, Task>,
    task_q: Query<&Task>,
    mut actor_q: Query<&mut Actor>,
) {
    let task = try_res_s!(task_q.get(trigger.entity()));
    let mut actor = try_res_s!(actor_q.get_mut(task.actor));
    if actor.task == Some(trigger.entity()) {
        actor.task = None;
    }
}

pub fn actor_removed(
    trigger: Trigger<OnReplace, Task>,
    mut commands: Commands,
    actor_q: Query<&Actor>,
) {
    let actor = try_res_s!(actor_q.get(trigger.entity()));
    if let Some(task) = actor.task {
        commands.entity(task).try_despawn_recursive();
    }
}
