//! Adapter for the `findmnt` command.

use crate::prelude::*;

/// Check that a partition is not already mounted.
pub trait CheckNotMounted: Send + Sync {
    /// Return `Ok(())` if the mount path is not mounted, or an error if it is.
    fn check_not_mounted(&self, mount_path: &Path) -> Result<(), Report<CheckNotMountedError>>;
}

/// Adapter wrapping the `findmnt` CLI tool.
#[derive(FromServices)]
pub struct FindmntAdapter;

impl CheckNotMounted for FindmntAdapter {
    fn check_not_mounted(&self, mount_path: &Path) -> Result<(), Report<CheckNotMountedError>> {
        let response = Command::new("findmnt")
            .arg("--noheadings")
            .arg(mount_path)
            .output()
            .change_context(CheckNotMountedError::Run)?
            .to_response();
        if response.status.success() {
            Err(Report::new(CheckNotMountedError::AlreadyMounted).attach_path(mount_path))
        } else {
            Ok(())
        }
    }
}

/// Errors returned by [`CheckNotMounted::check_not_mounted`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CheckNotMountedError {
    /// Unable to start `findmnt` or exchange data with it.
    #[error("Unable to run `findmnt`")]
    Run,
    /// The mount path is already mounted.
    #[error("Partition is already mounted")]
    AlreadyMounted,
}
