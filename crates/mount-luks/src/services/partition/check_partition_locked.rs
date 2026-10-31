//! Adapter for checking if a partition is locked.

use crate::prelude::*;

/// Check that the partition is not already unlocked.
pub trait CheckPartitionLocked: Send + Sync {
    /// Return `Ok(())` if the partition is locked, or an error if already unlocked.
    fn check_partition_locked(&self) -> Result<(), Report<CheckPartitionLockedError>>;
}

/// Adapter that checks the real filesystem for mapper device existence.
#[derive(FromServices)]
pub struct PartitionLockedAdapter {
    /// Configuration options containing the mapper name.
    options: Arc<Options>,
}

impl CheckPartitionLocked for PartitionLockedAdapter {
    fn check_partition_locked(&self) -> Result<(), Report<CheckPartitionLockedError>> {
        let mapper_path = self.options.get_mapper_path();
        if mapper_path.exists() {
            let report = Report::new(CheckPartitionLockedError::Unlocked).attach_path(&mapper_path);
            Err(report)
        } else {
            Ok(())
        }
    }
}

/// Errors returned by [`CheckPartitionLocked`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CheckPartitionLockedError {
    /// The mapper device already exists.
    #[error("Partition is already unlocked")]
    Unlocked,
}
