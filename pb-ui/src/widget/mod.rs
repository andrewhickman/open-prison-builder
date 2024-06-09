use bevy::{ecs::system::EntityCommands, prelude::*};

pub mod button;
pub mod error;
pub mod input;
pub mod panel;
pub mod spinner;

pub(crate) struct UiBuilder<'w, 's> {
    commands: Commands<'w, 's>,
    entity: Entity,
}

impl<'w, 's> UiBuilder<'w, 's> {
    pub fn new(commands: Commands<'w, 's>, entity: Entity) -> Self {
        UiBuilder { commands, entity }
    }

    pub fn reborrow(&mut self) -> UiBuilder<'w, '_> {
        UiBuilder::new(self.commands.reborrow(), self.entity)
    }

    pub fn spawn(&mut self, bundle: impl Bundle) -> UiBuilder<'w, '_> {
        let child = self.commands.spawn(bundle).set_parent(self.entity).id();
        UiBuilder::new(self.commands.reborrow(), child)
    }

    pub fn container(&mut self, style: Style) -> UiBuilder<'w, '_> {
        self.spawn(NodeBundle { style, ..default() })
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> UiBuilder<'w, '_> {
        self.commands.entity(self.entity).insert(bundle);
        self.reborrow()
    }

    pub fn id(&self) -> Entity {
        self.entity
    }
}

impl<'s> From<&'s mut EntityCommands<'s>> for UiBuilder<'s, 's> {
    fn from(commands: &'s mut EntityCommands<'s>) -> Self {
        UiBuilder {
            entity: commands.id(),
            commands: commands.commands(),
        }
    }
}
