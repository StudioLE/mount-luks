//! Stub for checking mount point existence.

use crate::prelude::*;

/// [`CheckMountExists`] returning a fixed result or [`CheckMountExistsError`].
pub struct CheckMountExistsStub {
    /// Value returned by [`CheckMountExists::check_mount_exists`].
    pub result: Result<(), CheckMountExistsError>,
}

impl FromServices for CheckMountExistsStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl CheckMountExists for CheckMountExistsStub {
    fn check_mount_exists(&self) -> Result<(), Report<CheckMountExistsError>> {
        self.result.map_err(Report::new)
    }
}
