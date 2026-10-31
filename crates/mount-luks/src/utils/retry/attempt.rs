//! Operations that [`Retry`] can repeat.

use crate::prelude::*;
use std::error::Error as StdError;

/// An operation that [`Retry`] can repeat after a retryable failure.
pub trait Attempt: Send + Sync {
    /// Input passed to every attempt.
    type Input: Copy;
    /// Output of a successful attempt.
    type Output;
    /// Error context of a failed attempt.
    type Error: StdError + Send + Sync + 'static;

    /// Run a single attempt.
    fn attempt(&self, input: Self::Input) -> Result<Self::Output, Report<Self::Error>>;

    /// Can attempting again succeed after this failure?
    fn is_retryable(&self, report: &Report<Self::Error>) -> bool;

    /// Options controlling the number of attempts and the delay between them.
    fn options(&self) -> RetryOptions {
        RetryOptions::default()
    }
}
