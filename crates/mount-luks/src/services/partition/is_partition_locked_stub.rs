//! Stub for checking if a partition is locked.

use crate::prelude::*;

/// [`IsPartitionLocked`] returning a fixed result or [`PartitionUnlocked`].
pub struct IsPartitionLockedStub {
    /// Value returned by [`IsPartitionLocked::is_partition_locked`].
    pub result: Result<(), PartitionUnlocked>,
}

impl FromServices for IsPartitionLockedStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl IsPartitionLocked for IsPartitionLockedStub {
    fn is_partition_locked(&self) -> Result<(), Report<PartitionUnlocked>> {
        self.result.map_err(Report::new)
    }
}
