//! Stub for listing persistent TPM handles.

use crate::prelude::*;

/// [`GetHandles`] returning a fixed [`Vec`] or [`GetHandlesError`].
pub struct GetHandlesStub {
    /// Value returned by [`GetHandles::get_handles`].
    pub handles: Result<Vec<PersistentHandle>, GetHandlesError>,
}

impl FromServices for GetHandlesStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            handles: Ok(Vec::new()),
        })
    }
}

impl GetHandles for GetHandlesStub {
    fn get_handles(&self) -> Result<Vec<PersistentHandle>, Report<GetHandlesError>> {
        self.handles.clone().map_err(Report::new)
    }
}
