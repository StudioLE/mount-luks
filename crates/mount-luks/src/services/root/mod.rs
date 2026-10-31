//! Root privilege checks.

mod check_root;
#[cfg(test)]
mod check_root_stub;

pub use check_root::*;
#[cfg(test)]
pub use check_root_stub::*;
