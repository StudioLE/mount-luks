//! Stub for unsealing data from a TPM object.

use crate::prelude::*;

/// [`Unseal`] returning a fixed [`String`] or [`UnsealError`].
pub struct UnsealStub {
    /// Value returned by [`Unseal::unseal`].
    pub key: Result<String, UnsealError>,
}

impl FromServices for UnsealStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            key: Ok(String::from("fixture-tpm-key")),
        })
    }
}

impl Unseal for UnsealStub {
    fn unseal(&self, _handle: PersistentHandle) -> Result<String, Report<UnsealError>> {
        self.key.clone().map_err(Report::new)
    }
}
