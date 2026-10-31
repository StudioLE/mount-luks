//! Adapter for checking mount point existence.

use crate::prelude::*;

/// Check that the mount point directory exists.
#[cfg_attr(test, mockall::automock)]
pub trait CheckMountExists: Send + Sync {
    /// Return `Ok(())` if the mount point exists, or an error otherwise.
    fn check_mount_exists(&self) -> Result<(), Report<NoMount>>;
}

/// Adapter that checks the real filesystem for mount point existence.
#[derive(FromServices)]
pub struct MountExistsAdapter {
    /// Configuration options containing the mount path.
    options: Arc<Options>,
}

impl CheckMountExists for MountExistsAdapter {
    fn check_mount_exists(&self) -> Result<(), Report<NoMount>> {
        if self.options.mount_path.exists() {
            Ok(())
        } else {
            Err(Report::new(NoMount).attach_path(&self.options.mount_path))
        }
    }
}

/// Error returned by [`CheckMountExists`] when the mount point does not exist.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Mount point does not exist")]
pub struct NoMount;
