//! Stub for checking root privileges.

use crate::prelude::*;

/// [`IsRoot`] returning a fixed result or [`IsRootError`].
pub struct IsRootStub {
    /// Value returned by [`IsRoot::is_root`].
    pub result: Result<(), IsRootError>,
}

impl FromServices for IsRootStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl IsRoot for IsRootStub {
    fn is_root(&self) -> Result<(), Report<IsRootError>> {
        self.result.map_err(Report::new)
    }
}
