//! Adapter for the `findmnt` command.

use crate::prelude::*;

/// Check that a partition is not already mounted.
pub trait FindMount: Send + Sync {
    /// Return `Ok(())` if the mount path is not mounted, or an error if it is.
    fn check_not_mounted(&self, mount_path: &Path) -> Result<(), Report<FindMountError>>;
}

/// Adapter wrapping the `findmnt` CLI tool.
#[derive(FromServices)]
pub struct FindmntAdapter;

impl FindMount for FindmntAdapter {
    fn check_not_mounted(&self, mount_path: &Path) -> Result<(), Report<FindMountError>> {
        let response = Command::new("findmnt")
            .arg("--noheadings")
            .arg(mount_path)
            .output()
            .expect("should be able to execute `findmnt`")
            .to_response();
        if response.status.success() {
            Err(Report::new(FindMountError::AlreadyMounted).attach_path(mount_path))
        } else {
            Ok(())
        }
    }
}

/// Errors returned by [`FindMount::check_not_mounted`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum FindMountError {
    /// The mount path is already mounted.
    #[error("Partition is already mounted")]
    AlreadyMounted,
}
