//! Adapter for the `cryptsetup` command.

mod add_key;
#[cfg(test)]
mod add_key_stub;
mod check_key;
#[cfg(test)]
mod check_key_stub;
mod cryptsetup_adapter;
mod is_luks;
#[cfg(test)]
mod is_luks_stub;
mod unlock;
#[cfg(test)]
mod unlock_stub;

pub use add_key::*;
#[cfg(test)]
pub use add_key_stub::*;
pub use check_key::*;
#[cfg(test)]
pub use check_key_stub::*;
pub use cryptsetup_adapter::*;
pub use is_luks::*;
#[cfg(test)]
pub use is_luks_stub::*;
pub use unlock::*;
#[cfg(test)]
pub use unlock_stub::*;
