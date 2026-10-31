//! Adapter for the `tpm2-tools` CLI.

use crate::prelude::*;

/// Adapter wrapping `tpm2-tools` CLI commands.
#[derive(FromServices)]
pub struct Tpm2Adapter;
