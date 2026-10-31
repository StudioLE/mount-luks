//! Stub for adding a key to a LUKS partition.

use crate::prelude::*;

/// [`AddKey`] returning a fixed result or [`AddKeyError`].
pub struct AddKeyStub {
    /// Value returned by [`AddKey::add_key`].
    pub result: Result<(), AddKeyError>,
}

impl FromServices for AddKeyStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl AddKey for AddKeyStub {
    fn add_key(
        &self,
        _partition: &Path,
        _existing_key: &str,
        _new_key: &str,
    ) -> Result<(), Report<AddKeyError>> {
        self.result.map_err(Report::new)
    }
}
