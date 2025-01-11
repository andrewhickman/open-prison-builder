use bevy::{
    ecs::system::{EntityCommands, IntoObserverSystem},
    picking::focus::PickingInteraction,
    prelude::*,
    ui::FocusPolicy,
};

pub mod button;
pub mod disabled;
pub mod error;
pub mod form;
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

    pub fn container(&mut self, style: Node) -> UiBuilder<'w, '_> {
        self.spawn((style, FocusPolicy::Pass, PickingBehavior::IGNORE))
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> UiBuilder<'w, '_> {
        self.commands.entity(self.entity).insert(bundle);
        self.reborrow()
    }

    pub fn observe<E, B, M>(
        &mut self,
        system: impl IntoObserverSystem<E, B, M>,
    ) -> UiBuilder<'w, '_>
    where
        E: Event,
        B: Bundle,
    {
        self.commands.entity(self.entity).observe(system);
        self.reborrow()
    }

    pub fn on_click<M>(
        &mut self,
        system: impl IntoObserverSystem<Pointer<Click>, (), M>,
    ) -> UiBuilder<'w, '_> {
        self.insert((PickingBehavior::default(), PickingInteraction::None));
        self.observe(system)
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
