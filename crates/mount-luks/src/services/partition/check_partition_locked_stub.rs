//! Stub for checking if a partition is locked.

use crate::prelude::*;

/// [`CheckPartitionLocked`] returning a fixed result or [`CheckPartitionLockedError`].
pub struct CheckPartitionLockedStub {
    /// Value returned by [`CheckPartitionLocked::check_partition_locked`].
    pub result: Result<(), CheckPartitionLockedError>,
}

impl FromServices for CheckPartitionLockedStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl CheckPartitionLocked for CheckPartitionLockedStub {
    fn check_partition_locked(&self) -> Result<(), Report<CheckPartitionLockedError>> {
        self.result.map_err(Report::new)
    }
}
