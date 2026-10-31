//! Adapter for checking if a partition is locked.

use crate::prelude::*;

/// Check that the partition is not already unlocked.
pub trait IsPartitionLocked: Send + Sync {
    /// Return `Ok(())` if the partition is locked, or an error if already unlocked.
    fn is_partition_locked(&self) -> Result<(), Report<IsPartitionLockedError>>;
}

/// Adapter that checks the real filesystem for mapper device existence.
#[derive(FromServices)]
pub struct PartitionLockedAdapter {
    /// Configuration options containing the mapper name.
    options: Arc<Options>,
}

impl IsPartitionLocked for PartitionLockedAdapter {
    fn is_partition_locked(&self) -> Result<(), Report<IsPartitionLockedError>> {
        let mapper_path = self.options.get_mapper_path();
        if mapper_path.exists() {
            let report = Report::new(IsPartitionLockedError::Unlocked).attach_path(&mapper_path);
            Err(report)
        } else {
            Ok(())
        }
    }
}

/// Errors returned by [`IsPartitionLocked`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum IsPartitionLockedError {
    /// The mapper device already exists.
    #[error("Partition is already unlocked")]
    Unlocked,
}
