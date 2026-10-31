//! Stub for validating a key against a LUKS partition.

use crate::prelude::*;

/// [`CheckKey`] returning a fixed result or [`CheckKeyError`].
pub struct CheckKeyStub {
    /// Value returned by [`CheckKey::check_key`].
    pub result: Result<(), CheckKeyError>,
}

impl FromServices for CheckKeyStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl CheckKey for CheckKeyStub {
    fn check_key(&self, _partition: &Path, _key: &str) -> Result<(), Report<CheckKeyError>> {
        self.result.map_err(Report::new)
    }
}
