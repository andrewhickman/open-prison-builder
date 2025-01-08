pub mod callback;

pub use anyhow;
pub use tracing;

use bevy::{
    ecs::system::{BoxedSystem, IntoSystem, ResMut},
    state::state::{FreelyMutableState, NextState},
};

pub use self::callback::{run_system_cached, run_system_cached_with, spawn_io, CallbackPlugin};

use std::fmt::Write;

pub fn set_state<S>(state: S) -> BoxedSystem
where
    S: FreelyMutableState + Clone,
{
    Box::new(IntoSystem::into_system(
        move |mut next_state: ResMut<NextState<S>>| next_state.set(state.clone()),
    ))
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
