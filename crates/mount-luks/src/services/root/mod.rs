//! Root privilege checks.

mod is_root;
#[cfg(test)]
mod is_root_stub;

pub use is_root::*;
#[cfg(test)]
pub use is_root_stub::*;
