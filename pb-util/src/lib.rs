pub mod callback;
pub mod math;
pub mod system;

pub use bevy;
pub use tracing;

use bevy::{
    ecs::{
        component::{Component, Mutable},
        entity::Entity,
        error::Result,
        system::{BoxedSystem, IntoSystem, ResMut},
        world::{Mut, World},
    },
    prelude::Command,
    state::state::{FreelyMutableState, NextState},
};
use tracing::warn;

pub use self::{
    callback::{CallbackPlugin, spawn_io},
    system::run_if,
};

use std::any::type_name;

pub fn set_state<S>(state: S) -> BoxedSystem<(), Result>
where
    S: FreelyMutableState + Clone,
{
    Box::new(IntoSystem::into_system(
        move |mut next_state: ResMut<NextState<S>>| {
            next_state.set(state.clone());
            Ok(())
        },
    ))
}

pub fn modify_component<C, F>(id: Entity, f: F) -> impl Command
where
    C: Component<Mutability = Mutable>,
    F: FnOnce(Mut<C>) + Send + 'static,
{
    move |world: &mut World| {
        if let Ok(mut entity) = world.get_entity_mut(id) {
            if let Some(component) = entity.get_mut() {
                f(component)
            } else {
                warn!("entity {id} did not have component {}", type_name::<C>())
            }
        } else {
            warn!(
                "entity {id} not found, failed to modify component {}",
                type_name::<C>()
            )
        }
    }
}

pub fn try_modify_component<C, F>(id: Entity, f: F) -> impl Command
where
    C: Component<Mutability = Mutable>,
    F: FnOnce(Mut<C>) + Send + 'static,
{
    move |world: &mut World| {
        if let Ok(mut entity) = world.get_entity_mut(id) {
            if let Some(component) = entity.get_mut() {
                f(component)
            }
        }
    }
}
