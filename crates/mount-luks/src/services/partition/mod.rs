//! Partition resolution and lock state.

mod is_partition_locked;
#[cfg(test)]
mod is_partition_locked_stub;
mod resolve_partition;
#[cfg(test)]
mod resolve_partition_stub;

pub use is_partition_locked::*;
#[cfg(test)]
pub use is_partition_locked_stub::*;
pub use resolve_partition::*;
#[cfg(test)]
pub use resolve_partition_stub::*;
