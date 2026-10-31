//! Mount point checks and mounting.

mod check_mount_exists;
#[cfg(test)]
mod check_mount_exists_stub;
mod check_not_mounted;
#[cfg(test)]
mod check_not_mounted_stub;
mod mount_partition;
#[cfg(test)]
mod mount_partition_stub;

pub use check_mount_exists::*;
#[cfg(test)]
pub use check_mount_exists_stub::*;
pub use check_not_mounted::*;
#[cfg(test)]
pub use check_not_mounted_stub::*;
pub use mount_partition::*;
#[cfg(test)]
pub use mount_partition_stub::*;
