use crate::prelude::*;
use std::error::Error as StdError;
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

/// Attach a [`Response`] to an error report as structured context.
pub trait AttachResponse {
    /// Attach stderr, stdout, and exit code from `response` as key-value pairs.
    fn attach_response(self, response: Response) -> Self;
}

impl<C: StdError + Send + Sync + 'static> AttachResponse for Report<C> {
    fn attach_response(mut self, response: Response) -> Self {
        if let Some(message) = response.error {
            self = self.attach("stderr", message);
        }
        if let Some(message) = response.output {
            self = self.attach("stdout", message);
        }
        if let Some(code) = response.status.code() {
            self = self.attach("exit", code);
        }
        self
    }
}

/// Convert a value into a [`Response`].
pub trait ToResponse {
    /// Convert `self` into a [`Response`].
    fn to_response(self) -> Response;
}

impl ToResponse for Output {
    fn to_response(self) -> Response {
        Response {
            error: to_string(&self.stderr),
            output: to_string(&self.stdout),
            status: self.status,
        }
    }
}

impl From<Output> for Response {
    fn from(output: Output) -> Self {
        output.to_response()
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
