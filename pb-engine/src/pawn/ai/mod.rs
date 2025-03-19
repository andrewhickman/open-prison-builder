pub mod path;

use bevy::prelude::*;
use pb_util::callback::CallbackSender;

#[derive(Default, Clone, Copy, Debug, Component)]
pub struct Task;

#[derive(Debug)]
pub enum TaskResult<T> {
    Ok(T),
    Cancelled,
    Err(Box<dyn std::error::Error + Send + Sync>),
}

pub struct PawnActor {
    id: Entity,
    target: Entity,
    sender: CallbackSender,
}

impl PawnActor {
    pub fn new(id: Entity, sender: CallbackSender) -> Self {
        PawnActor {
            id,
            target: id,
            sender,
        }
    }

    fn spawn_task(&self, task: impl Bundle) {
        let id = self.id;
        self.sender.send(move |world: &mut World| {
            let task = world.spawn(task).id();
            if let Ok(mut entity) = world.get_entity_mut(id) {
                entity.despawn_descendants();
                entity.replace_children(&[task]);
            } else {
                world.despawn(task);
            }
        });
    }
}
