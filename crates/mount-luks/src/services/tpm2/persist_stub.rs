//! Stub for making a TPM object persistent.

use crate::prelude::*;

/// [`Persist`] returning a fixed result or [`PersistError`].
pub struct PersistStub {
    /// Value returned by [`Persist::persist`].
    pub result: Result<(), PersistError>,
}

impl FromServices for PersistStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl Persist for PersistStub {
    fn persist(&self, _handle: PersistentHandle) -> Result<(), Report<PersistError>> {
        self.result.map_err(Report::new)
    }
}
