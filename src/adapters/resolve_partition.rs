//! Adapter for resolving PARTUUID to a device path.

use crate::prelude::*;

/// Adapter resolving a PARTUUID to a device path via `blkid`.
#[derive(FromServices)]
pub struct ResolvePartitionAdapter {
    /// Configuration options containing the PARTUUID.
    options: Arc<Options>,
}

/// Resolve a PARTUUID to a device path.
#[cfg_attr(test, mockall::automock)]
pub trait ResolvePartition: Send + Sync {
    /// Resolve the configured PARTUUID to a device path via `blkid`.
    fn resolve_partition(&self) -> Result<PathBuf, Report<ResolvePartitionError>>;
}

impl ResolvePartition for ResolvePartitionAdapter {
    fn resolve_partition(&self) -> Result<PathBuf, Report<ResolvePartitionError>> {
        let token = format!("PARTUUID={}", self.options.partuuid);
        let response = Command::new("blkid")
            .arg("-t")
            .arg(&token)
            .arg("-o")
            .arg("device")
            .output()
            .expect("should be able to execute `blkid`")
            .to_response();
        if response.status.success() {
            let path = response.output.ok_or_else(|| {
                Report::new(ResolvePartitionError::NotFound)
                    .attach("partuuid", self.options.partuuid.clone())
            })?;
            return Ok(PathBuf::from(path));
        }
        if response.status.code() == Some(2) {
            return Err(Report::new(ResolvePartitionError::NotFound)
                .attach("partuuid", self.options.partuuid.clone()));
        }
        Err(Report::new(ResolvePartitionError::Unexpected)
            .attach("partuuid", self.options.partuuid.clone())
            .attach_response(response))
    }
}

/// Errors returned by [`ResolvePartition::resolve_partition`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ResolvePartitionError {
    /// No device matches the configured PARTUUID.
    #[error("Partition not found for PARTUUID")]
    NotFound,
    /// Unexpected failure running `blkid`.
    #[error("Unable to resolve PARTUUID to device path")]
    Unexpected,
}
