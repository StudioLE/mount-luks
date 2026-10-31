//! Adapter for checking root privileges.

use crate::prelude::*;
use nix::unistd::Uid;

/// Check that the current process is running as root.
pub trait IsRoot: Send + Sync {
    /// Return `Ok(())` if the process is running as root, or an error otherwise.
    fn is_root(&self) -> Result<(), Report<IsRootError>>;
}

/// Adapter checking the effective user id of the current process.
#[derive(Default, FromServices)]
pub struct IsRootAdapter;

impl IsRoot for IsRootAdapter {
    fn is_root(&self) -> Result<(), Report<IsRootError>> {
        if Uid::effective().is_root() {
            Ok(())
        } else {
            Err(Report::new(IsRootError::NotRoot))
        }
    }
}

/// Errors returned by [`IsRoot::is_root`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum IsRootError {
    /// The process isn't running as root.
    #[error("Root is required")]
    NotRoot,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`IsRootAdapter::is_root`] when not running as root.
    #[test]
    fn is_root_adapter_is_root() {
        // Arrange
        let services = ServiceBuilder::new().with_test_services().build();
        let adapter = services.expect::<IsRootAdapter>();
        // Act
        let output = adapter.is_root();
        // Assert
        let report = output.expect_err("should not be root");
        assert_eq!(report.current_context(), &IsRootError::NotRoot);
    }
}
