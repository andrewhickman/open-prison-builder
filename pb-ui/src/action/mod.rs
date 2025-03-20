pub mod default;

use bevy::prelude::*;

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct Action;

pub fn action_added(
    trigger: Trigger<OnAdd, Action>,
    mut commands: Commands,
    action_q: Query<Entity, With<Action>>,
) {
    for id in &action_q {
        if trigger.entity() != id {
            commands.entity(id).despawn_recursive();
        }
    }
}

pub fn action_removed(
    trigger: Trigger<OnRemove, Action>,
    commands: Commands,
    action_q: Query<Entity, With<Action>>,
) {
    if action_q.iter().all(|id| id == trigger.entity()) {
        default::spawn(commands);
    }
}
