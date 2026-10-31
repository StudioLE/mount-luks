//! Extension traits for [`Command`].

use crate::prelude::*;
use std::io::{Result as IoResult, Write};
use std::process::{Output, Stdio};

/// Run a [`Command`] with input written to its stdin.
pub trait OutputWithInput {
    /// Run the command with `input` on stdin and wait for its output.
    ///
    /// - Pipes stdin, stdout, and stderr
    fn output_with_input(&mut self, input: &str) -> IoResult<Output>;
}

impl OutputWithInput for Command {
    fn output_with_input(&mut self, input: &str) -> IoResult<Output> {
        let mut child = self
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(input.as_bytes())?;
        }
        child.wait_with_output()
    }
}
