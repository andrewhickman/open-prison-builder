pub use anyhow;
pub use tracing;

#[macro_export]
macro_rules! try_res {
    ($res:expr) => {
        match ($res) {
            Ok(val) => val,
            Err(error) => {
                let error = $crate::anyhow::Error::from(error);
                let error: &dyn std::error::Error = error.as_ref();
                $crate::tracing::error!(error);
                return;
            }
        }
    };
}
