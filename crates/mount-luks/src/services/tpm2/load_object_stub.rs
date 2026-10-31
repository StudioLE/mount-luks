//! Stub for loading a TPM object.

use crate::prelude::*;

/// [`LoadObject`] returning a fixed result or [`LoadObjectError`].
pub struct LoadObjectStub {
    /// Value returned by [`LoadObject::load_object`].
    pub result: Result<(), LoadObjectError>,
}

impl FromServices for LoadObjectStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl LoadObject for LoadObjectStub {
    fn load_object(&self) -> Result<(), Report<LoadObjectError>> {
        self.result.map_err(Report::new)
    }
}
