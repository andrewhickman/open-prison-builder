pub mod callback;

pub use anyhow;
pub use tracing;

pub use self::callback::{
    run_oneshot_system, run_oneshot_system_with_input, spawn_io, CallbackPlugin,
};

use bevy::ecs::query::QueryEntityError;

pub trait AsDynError {
    fn as_dyn_error(&self) -> &'_ (dyn std::error::Error + 'static);
}

impl AsDynError for anyhow::Error {
    fn as_dyn_error(&self) -> &'_ (dyn std::error::Error + 'static) {
        self.as_ref()
    }
}

macro_rules! impl_as_dyn_error {
    ($($error:ty),*) => {
        $(
            impl AsDynError for $error {
                fn as_dyn_error(&self) -> &'_ (dyn std::error::Error + 'static) {
                     self
                }
            }
        )*
    };
}

impl_as_dyn_error!(QueryEntityError);

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
