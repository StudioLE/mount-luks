//! Adapter for checking mount point existence.

use crate::prelude::*;

/// Check that the mount point directory exists.
pub trait CheckMountExists: Send + Sync {
    /// Return `Ok(())` if the mount point exists, or an error otherwise.
    fn check_mount_exists(&self) -> Result<(), Report<CheckMountExistsError>>;
}

/// Adapter that checks the real filesystem for mount point existence.
#[derive(FromServices)]
pub struct MountExistsAdapter {
    /// Configuration options containing the mount path.
    options: Arc<Options>,
}

impl CheckMountExists for MountExistsAdapter {
    fn check_mount_exists(&self) -> Result<(), Report<CheckMountExistsError>> {
        if self.options.mount_path.exists() {
            Ok(())
        } else {
            Err(Report::new(CheckMountExistsError::Missing).attach_path(&self.options.mount_path))
        }
    }
}

/// Errors returned by [`CheckMountExists`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CheckMountExistsError {
    /// The mount point doesn't exist.
    #[error("Mount point does not exist")]
    Missing,
}
