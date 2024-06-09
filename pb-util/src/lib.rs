pub mod callback;

pub use anyhow;
pub use tracing;

pub use self::callback::{
    run_oneshot_system, run_oneshot_system_with_input, spawn_io, CallbackPlugin,
};

pub trait AsDynError<Marker: ?Sized> {
    fn as_dyn_error(&self) -> &'_ (dyn std::error::Error + 'static);
}

impl AsDynError<anyhow::Error> for anyhow::Error {
    fn as_dyn_error(&self) -> &'_ (dyn std::error::Error + 'static) {
        self.as_ref()
    }
}

impl<T> AsDynError<dyn std::error::Error> for T
where
    T: std::error::Error + 'static,
{
    fn as_dyn_error(&self) -> &'_ (dyn std::error::Error + 'static) {
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
