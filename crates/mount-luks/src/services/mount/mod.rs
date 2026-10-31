//! Mount point checks and mounting.

mod check_mount_exists;
#[cfg(test)]
mod check_mount_exists_stub;
mod find_mount;
#[cfg(test)]
mod find_mount_stub;
mod mount_partition;
#[cfg(test)]
mod mount_partition_stub;

pub use check_mount_exists::*;
#[cfg(test)]
pub use check_mount_exists_stub::*;
pub use find_mount::*;
#[cfg(test)]
pub use find_mount_stub::*;
pub use mount_partition::*;
#[cfg(test)]
pub use mount_partition_stub::*;
