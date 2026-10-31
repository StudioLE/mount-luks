//! Stub for creating a TPM primary key.

use crate::prelude::*;

/// [`CreatePrimary`] returning a fixed result or [`CreatePrimaryError`].
pub struct CreatePrimaryStub {
    /// Value returned by [`CreatePrimary::create_primary`].
    pub result: Result<(), CreatePrimaryError>,
}

impl FromServices for CreatePrimaryStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl CreatePrimary for CreatePrimaryStub {
    fn create_primary(&self) -> Result<(), Report<CreatePrimaryError>> {
        self.result.map_err(Report::new)
    }
}
