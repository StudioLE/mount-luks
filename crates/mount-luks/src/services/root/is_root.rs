//! Adapter for checking root privileges.

use crate::prelude::*;
use nix::unistd::Uid;

/// Check that the current process is running as root.
#[cfg_attr(test, mockall::automock)]
pub trait IsRoot: Send + Sync {
    /// Return `Ok(())` if the process is running as root, or an error otherwise.
    fn is_root(&self) -> Result<(), Report<RootRequired>>;
}

/// Adapter checking the effective user id of the current process.
#[derive(Default, FromServices)]
pub struct IsRootAdapter;

impl IsRoot for IsRootAdapter {
    fn is_root(&self) -> Result<(), Report<RootRequired>> {
        if Uid::effective().is_root() {
            Ok(())
        } else {
            Err(Report::new(RootRequired))
        }
    }
}

/// Error returned by [`IsRoot::is_root`] when the process is not running as root.
#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Root is required")]
pub struct RootRequired;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _is_root() {
        // Arrange
        // Act
        let result = IsRootAdapter.is_root();
        // Assert
        let error = result.expect_err("should be an error");
        let context = error.current_context();
        assert_eq!(context, &RootRequired);
    }
}
