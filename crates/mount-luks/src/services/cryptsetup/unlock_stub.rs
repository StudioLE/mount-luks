//! Stub for unlocking a LUKS partition.

use crate::prelude::*;

/// [`Unlock`] returning a fixed result or [`UnlockError`].
pub struct UnlockStub {
    /// Value returned by [`Unlock::unlock`].
    pub result: Result<(), UnlockError>,
}

impl FromServices for UnlockStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl Unlock for UnlockStub {
    fn unlock(
        &self,
        _partition: &Path,
        _mapper: &str,
        _key: &str,
    ) -> Result<(), Report<UnlockError>> {
        self.result.map_err(Report::new)
    }
}
