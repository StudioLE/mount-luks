//! Adapters for filesystem path checks.

mod check_mount_exists;
mod check_partition_exists;
mod is_partition_locked;

pub use check_mount_exists::*;
pub use check_partition_exists::*;
pub use is_partition_locked::*;
