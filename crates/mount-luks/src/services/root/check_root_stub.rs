//! Stub for checking root privileges.

use crate::prelude::*;

/// [`CheckRoot`] returning a fixed result or [`CheckRootError`].
pub struct CheckRootStub {
    /// Value returned by [`CheckRoot::check_root`].
    pub result: Result<(), CheckRootError>,
}

impl FromServices for CheckRootStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl CheckRoot for CheckRootStub {
    fn check_root(&self) -> Result<(), Report<CheckRootError>> {
        self.result.map_err(Report::new)
    }
}
