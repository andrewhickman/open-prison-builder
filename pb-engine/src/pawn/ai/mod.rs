pub mod path;

use bevy::prelude::*;

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
) -> Result {
    let task = task_q.get(trigger.target())?;
    let mut actor = actor_q.get_mut(task.actor)?;
    if let Some(prev_task) = actor.task.replace(trigger.target()) {
        commands.entity(prev_task).despawn();
    }
    Ok(())
}

pub fn task_removed(
    trigger: Trigger<OnReplace, Task>,
    task_q: Query<&Task>,
    mut actor_q: Query<&mut Actor>,
) -> Result {
    let task = task_q.get(trigger.target())?;
    if let Ok(mut actor) = actor_q.get_mut(task.actor) {
        if actor.task == Some(trigger.target()) {
            actor.task = None;
        }
    }
    Ok(())
}

pub fn actor_removed(
    trigger: Trigger<OnReplace, Actor>,
    mut commands: Commands,
    actor_q: Query<&Actor>,
) -> Result {
    let actor = actor_q.get(trigger.target())?;
    if let Some(task) = actor.task {
        commands.entity(task).despawn();
    }
    Ok(())
}
