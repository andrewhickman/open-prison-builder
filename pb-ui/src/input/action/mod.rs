use bevy::prelude::*;

use crate::input::cancel::Cancellable;

#[derive(Default, Clone, Copy, Debug, Component)]
#[require(Cancellable)]
pub struct InputAction;

pub fn action_added(
    trigger: Trigger<OnAdd, InputAction>,
    mut commands: Commands,
    action_q: Query<Entity, With<InputAction>>,
) {
    for id in &action_q {
        if trigger.entity() != id {
            commands.entity(id).despawn_recursive();
        }
    }
}
