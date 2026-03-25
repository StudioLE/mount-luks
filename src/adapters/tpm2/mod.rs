//! Adapters for `tpm2-tools` commands.

use crate::prelude::*;

mod create_object;
mod create_policy;
mod create_primary;
mod get_handles;
mod load_object;
mod persist;
mod unseal;

pub use create_object::*;
pub use create_policy::*;
pub use create_primary::*;
pub use get_handles::*;
pub use load_object::*;
pub use persist::*;
pub use unseal::*;

/// Adapter wrapping `tpm2-tools` CLI commands.
#[derive(FromServices)]
pub struct Tpm2Adapter;
