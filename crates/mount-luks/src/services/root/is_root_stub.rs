//! Stub for checking root privileges.

use crate::prelude::*;

/// [`IsRoot`] returning a fixed result or [`RootRequired`].
pub struct IsRootStub {
    /// Value returned by [`IsRoot::is_root`].
    pub result: Result<(), RootRequired>,
}

impl FromServices for IsRootStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl IsRoot for IsRootStub {
    fn is_root(&self) -> Result<(), Report<RootRequired>> {
        self.result.map_err(Report::new)
    }
}
