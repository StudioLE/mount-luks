use crate::prelude::*;
#[allow(deprecated)]
use error_stack::Context;
use std::process::{ExitStatus, Output};

pub struct Response {
    pub error: Option<String>,
    pub output: Option<String>,
    pub status: ExitStatus,
}

pub trait AttachResponse {
    fn attach_response(self, response: Response) -> Self;
}

#[allow(deprecated)]
impl<C: Context> AttachResponse for Report<C> {
    fn attach_response(mut self, response: Response) -> Self {
        if let Some(message) = response.error {
            self = self.attach(format!("stderr: {message}"));
        }
        if let Some(message) = response.output {
            self = self.attach(format!("stdout: {message}"));
        }
        if let Some(code) = response.status.code() {
            self = self.attach(format!("exit: {code}"));
        }
        self
    }
}

pub trait ToResponse {
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
