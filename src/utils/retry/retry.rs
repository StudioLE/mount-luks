//! Repeat an operation after retryable failures.

use crate::prelude::*;
use std::any::type_name;
use std::thread::sleep;

/// Repeat a [`Attempt`] operation with exponential backoff.
///
/// - Retries only failures accepted by [`Attempt::is_retryable`]
/// - Waits [`RetryOptions::get_delay`] between attempts
pub struct Retry<T: ?Sized> {
    /// Operation to attempt.
    inner: Arc<T>,
}

impl<T: ?Sized + Attempt> Retry<T> {
    /// Create a new [`Retry`] for an operation.
    #[must_use]
    pub fn new(inner: Arc<T>) -> Self {
        Self { inner }
    }

    /// Execute the operation, retrying retryable failures.
    ///
    /// # Errors
    ///
    /// - Fails with the last error once it is not retryable or no attempts remain
    /// - Attaches `attempts` and `is_retryable`
    pub fn execute(&self, input: T::Input) -> Result<T::Output, Report<T::Error>> {
        let operation = type_name::<T>();
        let options = self.inner.options();
        let max_attempts = options.max_attempts.max(1);
        let mut attempt = 0;
        loop {
            attempt += 1;
            trace!(operation, attempt, max_attempts, "Attempting");
            let mut report = match self.inner.attempt(input) {
                Ok(output) => {
                    trace!(operation, attempts = attempt, "Attempt succeeded");
                    return Ok(output);
                }
                Err(report) => report,
            };
            let is_retryable = self.inner.is_retryable(&report);
            report = report
                .attach("attempts", attempt)
                .attach("is_retryable", is_retryable);
            if !is_retryable || attempt >= max_attempts {
                return Err(report);
            }
            let delay = options.get_delay(attempt);
            trace!(
                operation,
                attempt,
                delay_ms = delay.as_millis(),
                error = %report.current_context(),
                "Retrying"
            );
            sleep(delay);
        }
    }
}

impl<T: ?Sized + Attempt + 'static> FromServices for Retry<T> {
    type Error = ResolveError;

    fn from_services(services: &ServiceProvider) -> Result<Self, Report<Self::Error>> {
        Ok(Self::new(services.get_trait::<T>()?))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::time::Duration;

    #[test]
    fn retry_execute() {
        // Arrange
        let operation = FakeOperation::new(5, vec![Ok(1)]);
        let retry = Retry::new(operation.clone());

        // Act
        let output = retry.execute(());

        // Assert
        assert_eq!(output.expect("should succeed"), 1);
        assert_eq!(operation.calls(), 1);
    }

    #[test]
    fn retry_execute_after_retryable_failures() {
        // Arrange
        let operation = FakeOperation::new(
            5,
            vec![Err(FakeError::Retryable), Err(FakeError::Retryable), Ok(1)],
        );
        let retry = Retry::new(operation.clone());

        // Act
        let output = retry.execute(());

        // Assert
        assert_eq!(output.expect("should succeed"), 1);
        assert_eq!(operation.calls(), 3);
    }

    #[test]
    fn retry_execute_non_retryable() {
        // Arrange
        let operation = FakeOperation::new(5, vec![Err(FakeError::Fatal)]);
        let retry = Retry::new(operation.clone());

        // Act
        let output = retry.execute(());

        // Assert
        let report = output.expect_err("should fail");
        assert_eq!(*report.current_context(), FakeError::Fatal);
        assert_eq!(
            report.to_string(),
            "Fatal failure\n▷ attempts: 1\n▷ is_retryable: false"
        );
        assert_eq!(operation.calls(), 1);
    }

    #[test]
    fn retry_execute_exhausted() {
        // Arrange
        let operation = FakeOperation::new(5, vec![Err(FakeError::Retryable); 5]);
        let retry = Retry::new(operation.clone());

        // Act
        let output = retry.execute(());

        // Assert
        let report = output.expect_err("should fail");
        assert_eq!(*report.current_context(), FakeError::Retryable);
        assert_eq!(
            report.to_string(),
            "Retryable failure\n▷ attempts: 5\n▷ is_retryable: true"
        );
        assert_eq!(operation.calls(), 5);
    }

    #[test]
    fn retry_execute_zero_max_attempts() {
        // Arrange
        let operation = FakeOperation::new(0, vec![Err(FakeError::Retryable)]);
        let retry = Retry::new(operation.clone());

        // Act
        let output = retry.execute(());

        // Assert
        assert!(output.is_err());
        assert_eq!(operation.calls(), 1);
    }

    #[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
    enum FakeError {
        #[error("Retryable failure")]
        Retryable,
        #[error("Fatal failure")]
        Fatal,
    }

    /// [`Attempt`] returning queued results with no delay between attempts.
    struct FakeOperation {
        max_attempts: u32,
        results: Mutex<VecDeque<Result<u32, FakeError>>>,
        calls: AtomicU32,
    }

    impl FakeOperation {
        fn new(max_attempts: u32, results: Vec<Result<u32, FakeError>>) -> Arc<Self> {
            Arc::new(Self {
                max_attempts,
                results: Mutex::new(VecDeque::from(results)),
                calls: AtomicU32::new(0),
            })
        }

        fn calls(&self) -> u32 {
            self.calls.load(Ordering::SeqCst)
        }
    }

    impl Attempt for FakeOperation {
        type Input = ();
        type Output = u32;
        type Error = FakeError;

        fn attempt(&self, _input: ()) -> Result<u32, Report<FakeError>> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.results
                .lock()
                .expect("should lock results")
                .pop_front()
                .expect("should have a result for every attempt")
                .map_err(Report::new)
        }

        fn is_retryable(&self, report: &Report<FakeError>) -> bool {
            *report.current_context() == FakeError::Retryable
        }

        fn options(&self) -> RetryOptions {
            RetryOptions {
                max_attempts: self.max_attempts,
                initial_delay: Duration::ZERO,
                ..RetryOptions::default()
            }
        }
    }
}
