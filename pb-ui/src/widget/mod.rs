use bevy::{ecs::system::EntityCommands, prelude::*, ui::FocusPolicy};
use bevy_mod_picking::picking_core::Pickable;

pub mod button;
pub mod error;
pub mod form;
pub mod input;
pub mod panel;
pub mod spinner;

#[derive(Component, Debug, Clone, PartialEq, Eq)]
pub struct Disabled(pub bool);

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
        self.spawn((
            NodeBundle {
                style,
                focus_policy: FocusPolicy::Pass,
                ..default()
            },
            Pickable::IGNORE,
        ))
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> UiBuilder<'w, '_> {
        self.commands.entity(self.entity).insert(bundle);
        self.reborrow()
    }

    pub fn clear(&mut self) {
        self.commands.entity(self.entity).despawn_descendants();
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

impl Disabled {
    pub const ENABLED: Self = Disabled(false);
    pub const DISABLED: Self = Disabled(true);
}
