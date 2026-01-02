#![allow(deprecated)]
use crate::prelude::*;
use error_stack::Context;
use std::process::Output;

pub trait OkOrReport {
    fn ok_or<C: Context>(self, context: C) -> Result<(), Report<C>>;
}

impl OkOrReport for Output {
    fn ok_or<C: Context>(self, context: C) -> Result<(), Report<C>> {
        if self.status.success() {
            Ok(())
        } else {
            let response = self.to_response();
            let report = Report::new(context).attach_response(response);
            Err(report)
        }
    }
}
