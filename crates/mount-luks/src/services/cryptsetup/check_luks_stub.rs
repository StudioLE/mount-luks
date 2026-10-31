//! Stub for checking if a partition is encrypted with LUKS.

use crate::prelude::*;

/// [`CheckLuks`] returning a fixed result or [`CheckLuksError`].
pub struct CheckLuksStub {
    /// Value returned by [`CheckLuks::check_luks`].
    pub result: Result<(), CheckLuksError>,
}

impl FromServices for CheckLuksStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl CheckLuks for CheckLuksStub {
    fn check_luks(&self, _partition: &Path) -> Result<(), Report<CheckLuksError>> {
        self.result.map_err(Report::new)
    }
}
