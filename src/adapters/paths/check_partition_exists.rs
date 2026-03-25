//! Adapter for checking partition device existence.

use crate::prelude::*;

/// Check that the partition device exists.
#[cfg_attr(test, mockall::automock)]
pub trait CheckPartitionExists: Send + Sync {
    /// Return `Ok(())` if the partition device exists, or an error otherwise.
    fn check_partition_exists(&self) -> Result<(), Report<NoPartition>>;
}

/// Adapter that checks the real filesystem for partition device existence.
#[derive(FromServices)]
pub struct PartitionExistsAdapter {
    /// Configuration options containing the partition path.
    options: Arc<Options>,
}

impl CheckPartitionExists for PartitionExistsAdapter {
    fn check_partition_exists(&self) -> Result<(), Report<NoPartition>> {
        if self.options.partition_path.exists() {
            Ok(())
        } else {
            Err(Report::new(NoPartition).attach_path(&self.options.partition_path))
        }
    }
}

/// Error returned by [`CheckPartitionExists`] when the partition device does not exist.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Partition does not exist")]
pub struct NoPartition;
