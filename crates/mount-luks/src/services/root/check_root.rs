//! Adapter for checking root privileges.

use crate::prelude::*;
use nix::unistd::Uid;

/// Check that the current process is running as root.
pub trait CheckRoot: Send + Sync {
    /// Return `Ok(())` if the process is running as root, or an error otherwise.
    fn check_root(&self) -> Result<(), Report<CheckRootError>>;
}

/// Adapter checking the effective user id of the current process.
#[derive(Default, FromServices)]
pub struct UidAdapter;

impl CheckRoot for UidAdapter {
    fn check_root(&self) -> Result<(), Report<CheckRootError>> {
        if Uid::effective().is_root() {
            Ok(())
        } else {
            Err(Report::new(CheckRootError::NotRoot))
        }
    }
}

/// Errors returned by [`CheckRoot::check_root`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CheckRootError {
    /// The process isn't running as root.
    #[error("Root is required")]
    NotRoot,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`UidAdapter::check_root`] when not running as root.
    #[test]
    fn uid_adapter_check_root() {
        // Arrange
        let services = ServiceBuilder::new().with_test_services().build();
        let adapter = services.expect::<UidAdapter>();
        // Act
        let output = adapter.check_root();
        // Assert
        let report = output.expect_err("should not be root");
        assert_eq!(report.current_context(), &CheckRootError::NotRoot);
    }
}
