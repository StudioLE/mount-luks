//! Stub for checking if a path is already mounted.

use crate::prelude::*;

/// [`FindMount`] returning a fixed result or [`AlreadyMounted`].
pub struct FindMountStub {
    /// Value returned by [`FindMount::check_not_mounted`].
    pub result: Result<(), AlreadyMounted>,
}

impl FromServices for FindMountStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl FindMount for FindMountStub {
    fn check_not_mounted(&self, _mount_path: &Path) -> Result<(), Report<AlreadyMounted>> {
        self.result.map_err(Report::new)
    }
}
