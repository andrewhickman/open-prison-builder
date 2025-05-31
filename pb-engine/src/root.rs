use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Copy, Clone, Component, Reflect, Serialize, Deserialize)]
#[reflect(Component, Serialize, Deserialize)]
#[require(Transform, ChildOfRoot, Name::new(Root::type_path()))]
pub struct Root;

#[derive(Default, Copy, Clone, Component)]
pub struct ChildOfRoot;

pub fn child_added(
    trigger: Trigger<OnInsert, ChildOf>,
    mut commands: Commands,
    parent_q: Query<&ChildOf>,
    root_q: Query<Entity, With<ChildOfRoot>>,
) -> Result {
    let parent = parent_q.get(trigger.target())?;
    if root_q.contains(parent.parent()) {
        commands
            .entity(trigger.target())
            .insert_recursive::<Children>(ChildOfRoot);
    }

    Ok(())
}
