//! Adapter for the `findmnt` command.

use crate::prelude::*;

/// Check that a partition is not already mounted.
#[cfg_attr(test, mockall::automock)]
pub trait FindMount: Send + Sync {
    /// Return `Ok(())` if the mount path is not mounted, or an error if it is.
    fn check_not_mounted(&self, mount_path: &Path) -> Result<(), Report<AlreadyMounted>>;
}

/// Adapter wrapping the `findmnt` CLI tool.
#[derive(FromServices)]
pub struct FindmntAdapter;

impl FindMount for FindmntAdapter {
    fn check_not_mounted(&self, mount_path: &Path) -> Result<(), Report<AlreadyMounted>> {
        let response = Command::new("findmnt")
            .arg("--noheadings")
            .arg(mount_path)
            .output()
            .expect("should be able to execute `findmnt`")
            .to_response();
        if response.status.success() {
            Err(Report::new(AlreadyMounted).attach_path(mount_path))
        } else {
            Ok(())
        }
    }
}

/// Error returned by [`FindMount::check_not_mounted`] when the partition is already mounted.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Partition is already mounted")]
pub struct AlreadyMounted;
