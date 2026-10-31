//! Stub for creating a sealed TPM object.

use crate::prelude::*;

/// [`CreateObject`] returning a fixed result or [`CreateObjectError`].
pub struct CreateObjectStub {
    /// Value returned by [`CreateObject::create_object`].
    pub result: Result<(), CreateObjectError>,
}

impl FromServices for CreateObjectStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl CreateObject for CreateObjectStub {
    fn create_object(&self, _input: &str) -> Result<(), Report<CreateObjectError>> {
        self.result.map_err(Report::new)
    }
}
