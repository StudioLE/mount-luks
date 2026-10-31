//! Adapter for interactive password prompts.

use crate::prelude::*;
use rpassword::prompt_password;

/// Prompt the user for a password.
pub trait PromptPassword: Send + Sync {
    /// Display a message and read a password from the terminal.
    fn prompt(&self, message: &str) -> Result<String, Report<PromptPasswordError>>;
}

/// Adapter wrapping `rpassword::prompt_password`.
#[derive(FromServices)]
pub struct RpasswordAdapter;

impl PromptPassword for RpasswordAdapter {
    fn prompt(&self, message: &str) -> Result<String, Report<PromptPasswordError>> {
        prompt_password(message).change_context(PromptPasswordError::Read)
    }
}

/// Errors returned by [`PromptPassword::prompt`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum PromptPasswordError {
    /// Unable to read from the terminal.
    #[error("Unable to read password from prompt")]
    Read,
}
