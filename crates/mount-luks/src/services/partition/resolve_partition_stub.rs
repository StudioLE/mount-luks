//! Stub for resolving a PARTUUID to a device path.

use crate::prelude::*;

/// [`ResolvePartition`] returning a fixed [`PathBuf`] or [`ResolvePartitionError`].
pub struct ResolvePartitionStub {
    /// Value returned by [`ResolvePartition::resolve_partition`].
    pub partition: Result<PathBuf, ResolvePartitionError>,
}

impl FromServices for ResolvePartitionStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            partition: Ok(PathBuf::from("/dev/fixture")),
        })
    }
}

impl ResolvePartition for ResolvePartitionStub {
    fn resolve_partition(&self) -> Result<PathBuf, Report<ResolvePartitionError>> {
        self.partition.clone().map_err(Report::new)
    }
}
