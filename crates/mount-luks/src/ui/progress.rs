use crate::prelude::*;
use owo_colors::OwoColorize;

pub struct Progress {
    current: usize,
    total: usize,
}

impl Progress {
    pub fn new(total: usize) -> Self {
        Self { current: 0, total }
    }

    /// Print the start of a step, incrementing the shared counter.
    pub fn step(&mut self, message: &str) {
        self.increment();
        info!(
            "{}",
            format!("{}/{} {message}", self.current, self.total).dimmed()
        );
    }

    /// Print a success indicator for a completed step.
    #[expect(clippy::unused_self, reason = "consistency with step")]
    pub fn ok(&self, message: &str) {
        info!("{} {message}", CHECK.dimmed());
    }

    fn increment(&mut self) {
        self.current += 1;
    }
}
