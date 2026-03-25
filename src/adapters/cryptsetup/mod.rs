//! Adapter for the `cryptsetup` command.

use crate::prelude::*;

mod add_key;
mod check_key;
mod is_luks;
mod unlock;

pub use add_key::*;
pub use check_key::*;
pub use is_luks::*;
pub use unlock::*;

/// Adapter wrapping the `cryptsetup` CLI tool.
#[derive(FromServices)]
pub struct CryptsetupAdapter;
