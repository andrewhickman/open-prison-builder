pub mod callback;
pub mod math;
pub mod system;

pub use anyhow;
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

use std::{any::type_name, fmt::Write};

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

pub trait AsDynError<'a, Marker: ?Sized> {
    fn as_dyn_error(&self) -> &'_ (dyn std::error::Error + 'a);

    fn to_string_compact(&self) -> String {
        let mut error = self.as_dyn_error();

        let mut buf = error.to_string();
        while let Some(source) = error.source() {
            write!(buf, ": {}", source).unwrap();
            error = source;
        }

        buf
    }
}

impl<'a> AsDynError<'a, anyhow::Error> for anyhow::Error {
    fn as_dyn_error(&self) -> &'_ (dyn std::error::Error + 'a) {
        self.as_ref()
    }
}

impl<'a, T> AsDynError<'a, dyn std::error::Error> for T
where
    T: std::error::Error + 'a,
{
    fn as_dyn_error(&self) -> &'_ (dyn std::error::Error + 'a) {
        self
    }
}

#[macro_export]
macro_rules! try_res {
    ($res:expr) => {
        match ($res) {
            Ok(val) => val,
            Err(error) => {
                use $crate::AsDynError;
                $crate::tracing::error!(error = error.as_dyn_error());
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! try_res_s {
    ($res:expr) => {
        match ($res) {
            Ok(val) => val,
            Err(error) => {
                use $crate::AsDynError;
                $crate::tracing::error!(error = error.to_string_compact());
                return;
            }
        }
    };
}

#[macro_export]
macro_rules! try_opt {
    ($res:expr) => {
        match ($res) {
            Some(val) => val,
            None => return,
        }
    };
}
