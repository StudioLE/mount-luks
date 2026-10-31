//! Extension traits for [`Output`].

use crate::prelude::*;
use std::error::Error as StdError;
use std::process::Output;

/// Convert a [`std::process::Output`] into a `Result`, returning an error report on failure.
pub trait OkOrReport {
    /// Return `Ok(())` if the process exited successfully, or an error report with the given context.
    fn ok_or<C: StdError + Send + Sync + 'static>(self, context: C) -> Result<(), Report<C>>;
}

impl OkOrReport for Output {
    #[track_caller]
    fn ok_or<C: StdError + Send + Sync + 'static>(self, context: C) -> Result<(), Report<C>> {
        if self.status.success() {
            Ok(())
        } else {
            let response = self.to_response();
            let report = Report::new(context).attach_response(response);
            Err(report)
        }
    }
}

/// Convert a value into a [`Response`].
pub trait ToResponse {
    /// Convert `self` into a [`Response`].
    fn to_response(self) -> Response;
}

impl ToResponse for Output {
    fn to_response(self) -> Response {
        Response::from(self)
    }
}
