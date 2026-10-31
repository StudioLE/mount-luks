//! Stub for creating a TPM PCR policy.

use crate::prelude::*;

/// [`CreatePolicy`] returning a fixed result or [`CreatePolicyError`].
pub struct CreatePolicyStub {
    /// Value returned by [`CreatePolicy::create_policy`].
    pub result: Result<(), CreatePolicyError>,
}

impl FromServices for CreatePolicyStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self { result: Ok(()) })
    }
}

impl CreatePolicy for CreatePolicyStub {
    fn create_policy(&self) -> Result<(), Report<CreatePolicyError>> {
        self.result.map_err(Report::new)
    }
}
