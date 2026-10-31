//! Stub for prompting for a password.

use crate::prelude::*;

/// [`PromptPassword`] returning a fixed [`String`] or [`PromptPasswordError`].
pub struct PromptPasswordStub {
    /// Value returned by [`PromptPassword::prompt_password`].
    pub password: Result<String, PromptPasswordError>,
}

impl FromServices for PromptPasswordStub {
    type Error = Infallible;

    fn from_services(_services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self {
            password: Ok(String::from("fixture-password")),
        })
    }
}

impl PromptPassword for PromptPasswordStub {
    fn prompt_password(&self, _message: &str) -> Result<String, Report<PromptPasswordError>> {
        self.password.clone().map_err(Report::new)
    }
}
