//! Adapter for the `mount` command.

use crate::prelude::*;

/// Mount a partition.
pub trait MountPartition: Send + Sync {
    /// Mount a device at the given path.
    fn mount(&self, device: &Path, mount_path: &Path) -> Result<(), Report<MountPartitionError>>;
}

/// Adapter wrapping the `mount` CLI tool.
#[derive(FromServices)]
pub struct MountAdapter;

impl MountPartition for MountAdapter {
    fn mount(&self, device: &Path, mount_path: &Path) -> Result<(), Report<MountPartitionError>> {
        Command::new("mount")
            .arg(device)
            .arg(mount_path)
            .output()
            .expect("should be able to execute `mount`")
            .ok_or(MountPartitionError::Failed)
    }
}

/// Errors returned by [`MountPartition::mount`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum MountPartitionError {
    /// `mount` exited with an error.
    #[error("Failed to mount partition")]
    Failed,
}
