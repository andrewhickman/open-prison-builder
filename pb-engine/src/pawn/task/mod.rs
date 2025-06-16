pub mod path;

use bevy::prelude::*;

#[derive(Clone, Debug, Component)]
#[relationship(relationship_target = TaskStack)]
pub struct Task {
    target: Entity,
}

#[derive(Default, Clone, Debug, Component)]
#[relationship_target(relationship = Task, linked_spawn)]
pub struct TaskStack {
    tasks: Vec<Entity>,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct TaskSystems;

impl Task {
    pub fn new(target: Entity) -> Self {
        Task { target }
    }

    pub fn target(&self) -> Entity {
        self.target
    }
}
