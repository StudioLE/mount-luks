//! Adapter for checking root privileges.

use crate::prelude::*;

/// Check that the current process is running as root.
#[cfg_attr(test, mockall::automock)]
pub trait IsRoot: Send + Sync {
    /// Return `Ok(())` if the process is running as root, or an error otherwise.
    fn is_root(&self) -> Result<(), Report<RootRequired>>;
}

/// Adapter wrapping the `is_root` utility function.
#[derive(Default, FromServices)]
pub struct IsRootAdapter;

impl IsRoot for IsRootAdapter {
    fn is_root(&self) -> Result<(), Report<RootRequired>> {
        is_root()
    }
}
