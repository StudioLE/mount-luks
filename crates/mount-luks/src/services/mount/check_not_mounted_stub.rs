//! Stub for checking if a path is already mounted.

use crate::prelude::*;

/// [`CheckNotMounted`] returning a fixed result or [`CheckNotMountedError`].
pub struct CheckNotMountedStub {
    /// Value returned by [`CheckNotMounted::check_not_mounted`].
    pub result: Result<(), CheckNotMountedError>,
}

impl FromServices for CheckNotMountedStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl CheckNotMounted for CheckNotMountedStub {
    fn check_not_mounted(&self, _mount_path: &Path) -> Result<(), Report<CheckNotMountedError>> {
        self.result.map_err(Report::new)
    }
}
