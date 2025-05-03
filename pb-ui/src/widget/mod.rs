use std::borrow::Cow;

use bevy::{
    ecs::system::{EntityCommands, IntoObserverSystem},
    picking::hover::PickingInteraction,
    prelude::*,
    ui::FocusPolicy,
};
use pb_util::run_if;

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
        let child = self.commands.spawn((bundle, ChildOf(self.entity))).id();
        UiBuilder::new(self.commands.reborrow(), child)
    }

    pub fn container(&mut self, style: Node) -> UiBuilder<'w, '_> {
        self.spawn((style, FocusPolicy::Pass, Pickable::IGNORE))
    }

    pub fn insert(&mut self, bundle: impl Bundle) -> UiBuilder<'w, '_> {
        self.commands.entity(self.entity).insert(bundle);
        self.reborrow()
    }

    pub fn observe<E, M>(&mut self, system: impl IntoObserverSystem<E, (), M>) -> UiBuilder<'w, '_>
    where
        E: Event,
    {
        self.commands.entity(self.entity).observe(system);
        self.reborrow()
    }

    pub fn on_click<M, S>(&mut self, system: S) -> UiBuilder<'w, '_>
    where
        S: IntoSystem<Trigger<'static, Pointer<Click>>, (), M> + Send + 'static,
        M: 'static,
    {
        self.insert((Pickable::default(), PickingInteraction::None));
        self.observe(run_if(
            system,
            move |trigger: &mut Trigger<Pointer<Click>>| {
                if trigger.event().event.button == PointerButton::Primary {
                    trigger.propagate(false);
                    true
                } else {
                    false
                }
            },
        ))
    }

    pub fn clear(&mut self) {
        self.commands
            .entity(self.entity)
            .despawn_related::<Children>();
    }

    pub fn id(&self) -> Entity {
        self.entity
    }

    pub fn named(&mut self, name: impl Into<Cow<'static, str>>) -> UiBuilder<'w, '_> {
        self.insert(Name::new(name))
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
