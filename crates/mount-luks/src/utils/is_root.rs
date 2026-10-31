use crate::prelude::*;
use nix::unistd::Uid;

/// Check that the current process is running as root.
pub fn is_root() -> Result<(), Report<RootRequired>> {
    let is_root = Uid::effective().is_root();
    if is_root {
        Ok(())
    } else {
        Err(Report::new(RootRequired))
    }
}

/// Error returned by [`is_root`] when the process is not running as root.
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
        let result = is_root();
        // Assert
        let error = result.expect_err("should be an error");
        let context = error.current_context();
        assert_eq!(context, &RootRequired);
    }
}
