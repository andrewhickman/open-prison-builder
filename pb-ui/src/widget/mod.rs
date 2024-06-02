use bevy::prelude::*;

mod spinner;

pub(crate) struct UiBuilder<'a> {
    commands: Commands<'a, 'a>,
    entity: Entity,
}

impl<'a> UiBuilder<'a> {
    pub fn new(commands: Commands<'a, 'a>, entity: Entity) -> Self {
        UiBuilder { commands, entity }
    }

    pub fn spawn(&mut self, bundle: impl Bundle) -> UiBuilder<'_> {
        let child = self.commands.spawn(bundle).set_parent(self.entity).id();
        UiBuilder {
            commands: self.commands.reborrow(),
            entity: child,
        }
    }

    pub fn id(&self) -> Entity {
        self.entity
    }
}
