//! Captured output of a child process.

use std::process::{ExitStatus, Output};

/// Captured output from a child process.
pub struct Response {
    /// Trimmed stderr output, or `None` if empty.
    pub error: Option<String>,
    /// Trimmed stdout output, or `None` if empty.
    pub output: Option<String>,
    /// Exit status of the process.
    pub status: ExitStatus,
}

impl From<Output> for Response {
    fn from(output: Output) -> Self {
        Response {
            error: to_string(&output.stderr),
            output: to_string(&output.stdout),
            status: output.status,
        }
    }
}

fn to_string(buffer: &[u8]) -> Option<String> {
    let mut output = String::from_utf8_lossy(buffer).to_string();
    output.trim().to_owned().clone_into(&mut output);
    if output.is_empty() {
        None
    } else {
        Some(output)
    }
}
