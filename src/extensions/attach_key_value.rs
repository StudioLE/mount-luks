#![allow(deprecated)]
use crate::prelude::*;
use error_stack::Context;

pub trait AttachKeyValueToReport<C: Context> {
    fn attach_key_value(self, key: &str, value: &str) -> Report<C>;
}

impl<C: Context> AttachKeyValueToReport<C> for Report<C> {
    fn attach_key_value(self, key: &str, value: &str) -> Report<C> {
        self.attach(format!("{key}: {value}"))
    }
}

pub trait AttachKeyValueToResult<T, C: Context> {
    fn attach_key_value(self, key: &str, value: &str) -> Result<T, Report<C>>;
}

impl<T, C: Context> AttachKeyValueToResult<T, C> for Result<T, Report<C>> {
    fn attach_key_value(self, key: &str, value: &str) -> Result<T, Report<C>> {
        self.map_err(|report| report.attach_key_value(key, value))
    }
}
