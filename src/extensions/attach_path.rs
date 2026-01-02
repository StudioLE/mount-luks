#![allow(deprecated)]
use crate::extensions::attach_key_value::AttachKeyValueToReport;
use crate::prelude::*;
use error_stack::Context;

pub trait AttachPath<C: Context> {
    fn attach_path(self, path: &Path) -> Report<C>;
}

impl<C: Context> AttachPath<C> for Report<C> {
    fn attach_path(self, path: &Path) -> Report<C> {
        self.attach_key_value("Path", &path.display().to_string())
    }
}

pub trait AttachPathToResult<T, C: Context> {
    fn attach_path(self, path: &Path) -> Result<T, Report<C>>;
}

impl<T, C: Context> AttachPathToResult<T, C> for Result<T, Report<C>> {
    fn attach_path(self, path: &Path) -> Result<T, Report<C>> {
        self.map_err(|report| report.attach_path(path))
    }
}
