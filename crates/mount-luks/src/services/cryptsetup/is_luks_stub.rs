//! Stub for checking if a partition is encrypted with LUKS.

use crate::prelude::*;

/// [`IsLuks`] returning a fixed result or [`IsLuksError`].
pub struct IsLuksStub {
    /// Value returned by [`IsLuks::is_luks`].
    pub result: Result<(), IsLuksError>,
}

impl FromServices for IsLuksStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl IsLuks for IsLuksStub {
    fn is_luks(&self, _partition: &Path) -> Result<(), Report<IsLuksError>> {
        self.result.map_err(Report::new)
    }
}
