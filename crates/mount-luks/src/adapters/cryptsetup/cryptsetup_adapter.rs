use crate::prelude::*;

/// Adapter wrapping the `cryptsetup` CLI tool.
#[derive(FromServices)]
pub struct CryptsetupAdapter;
