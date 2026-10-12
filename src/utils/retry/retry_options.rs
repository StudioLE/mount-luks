//! Options controlling how [`Retry`] repeats an operation.

use crate::prelude::*;
use std::time::Duration;

/// Options controlling how [`Retry`] repeats an operation.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RetryOptions {
    /// Maximum number of attempts, including the first.
    ///
    /// - Values below `1` are treated as `1`
    ///
    /// Default: `5`
    pub max_attempts: u32,
    /// Delay after the first failed attempt.
    ///
    /// Default: `50ms`
    pub initial_delay: Duration,
    /// Factor applied to the delay after each failed attempt.
    ///
    /// Default: `2.0`
    pub multiplier: f32,
}

impl Default for RetryOptions {
    fn default() -> Self {
        Self {
            max_attempts: 5,
            initial_delay: Duration::from_millis(50),
            multiplier: 2.0,
        }
    }
}

impl RetryOptions {
    /// Delay to wait after a failed attempt.
    ///
    /// - Calculates `initial_delay × multiplier^(attempt − 1)`
    ///
    /// # Panics
    ///
    /// - Panics if the delay is negative, not finite, or overflows [`Duration`]
    #[must_use]
    pub fn get_delay(&self, attempt: u32) -> Duration {
        let exponent = i32::try_from(attempt.saturating_sub(1)).expect("attempt should fit in i32");
        self.initial_delay
            .mul_f64(f64::from(self.multiplier).powi(exponent))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_options_get_delay() {
        // Arrange
        let options = RetryOptions::default();

        // Act
        let delays: Vec<Duration> = (1..=4).map(|attempt| options.get_delay(attempt)).collect();

        // Assert
        assert_eq!(
            delays,
            vec![
                Duration::from_millis(50),
                Duration::from_millis(100),
                Duration::from_millis(200),
                Duration::from_millis(400),
            ]
        );
    }

    #[test]
    fn retry_options_get_delay_constant() {
        // Arrange
        let options = RetryOptions {
            multiplier: 1.0,
            ..RetryOptions::default()
        };

        // Act
        let delays: Vec<Duration> = (1..=4).map(|attempt| options.get_delay(attempt)).collect();

        // Assert
        assert_eq!(delays, vec![Duration::from_millis(50); 4]);
    }
}
