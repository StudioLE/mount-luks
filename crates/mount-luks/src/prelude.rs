//! Common imports used throughout the crate.

pub use crate::app::Cli;
pub(crate) use crate::app::*;
pub(crate) use crate::commands::*;
pub(crate) use crate::extensions::*;
pub(crate) use crate::services::*;
#[cfg(test)]
pub(crate) use crate::testing::*;
pub(crate) use std::convert::Infallible;
pub(crate) use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
pub(crate) use std::path::{Path, PathBuf};
pub(crate) use std::process::Command;
pub(crate) use std::str::FromStr;
pub(crate) use std::sync::Arc;
pub(crate) use studiole_di::prelude::*;
pub(crate) use studiole_logging::prelude::*;
pub(crate) use studiole_report::prelude::{Attach, Report, ResultExt, StructuredError};
pub(crate) use thiserror::Error;
#[expect(
    unused_imports,
    reason = "prelude exports all log macros for convenience"
)]
pub(crate) use tracing::{debug, error, info, trace, warn};

#[cfg(test)]
pub(crate) use insta::assert_snapshot;
