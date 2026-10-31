//! Stub for mounting a partition.

use crate::prelude::*;

/// [`MountPartition`] returning a fixed result or [`MountPartitionError`].
pub struct MountPartitionStub {
    /// Value returned by [`MountPartition::mount`].
    pub result: Result<(), MountPartitionError>,
}

impl FromServices for MountPartitionStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl MountPartition for MountPartitionStub {
    fn mount(&self, _device: &Path, _mount_path: &Path) -> Result<(), Report<MountPartitionError>> {
        self.result.map_err(Report::new)
    }
}
