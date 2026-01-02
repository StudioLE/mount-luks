use crate::prelude::*;
use nix::unistd::Uid;

pub fn is_root() -> Result<(), Report<RootRequired>> {
    let is_root = Uid::effective().is_root();
    if is_root { Ok(()) } else { bail!(RootRequired) }
}

#[derive(Clone, Copy, Debug, Error, PartialEq)]
#[error("Root is required")]
pub struct RootRequired;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn _is_root() {
        assert!(is_root().is_ok(), "Root is required to run this test");

        // Arrange
        // Act
        let result = is_root();

        // Assert
        assert!(result.is_ok());
    }
}
