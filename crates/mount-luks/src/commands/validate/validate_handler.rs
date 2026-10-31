//! Handler for the `validate` subcommand.

use crate::prelude::*;

const TOTAL_STEPS: usize = 4;

/// Handler for validating a LUKS key against a partition.
#[derive(FromServices)]
pub struct ValidateHandler {
    /// Adapter for checking root privileges.
    is_root: Arc<dyn IsRoot>,
    /// Adapter for resolving PARTUUID to a device path.
    resolve_partition: Arc<dyn ResolvePartition>,
    /// Adapter for checking if a key is valid for a LUKS partition.
    check_key: Arc<dyn CheckKey>,
    /// Adapter for retrieving the composite key.
    get_key: Arc<dyn GetKey>,
}

impl ValidateHandler {
    /// Execute the validate workflow.
    pub fn execute(&self) -> Result<(), Report<ValidateError>> {
        let mut progress = Progress::new(TOTAL_STEPS);

        progress.step("Checking if root");
        self.is_root
            .is_root()
            .change_context(ValidateError::CheckRoot)?;
        progress.ok("Access granted");

        progress.step("Resolving partition");
        let partition = self
            .resolve_partition
            .resolve_partition()
            .change_context(ValidateError::ResolvePartition)?;
        progress.ok(&format!("Resolved to {}", partition.display()));

        progress.step("Getting key");
        let key = self.get_key.get().change_context(ValidateError::GetKey)?;
        progress.ok("Key retrieved");

        progress.step("Validating key against partition");
        self.check_key
            .check_key(&partition, &key)
            .change_context(ValidateError::CheckKey)?;
        progress.ok("Key is valid");

        Ok(())
    }
}

/// Errors returned by [`ValidateHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum ValidateError {
    /// Unable to confirm the process is running as root.
    #[error("Unable to confirm root privileges")]
    CheckRoot,
    /// Unable to resolve the PARTUUID to a device path.
    #[error("Unable to resolve partition")]
    ResolvePartition,
    /// Unable to get the key.
    #[error("Unable to get key")]
    GetKey,
    /// Unable to validate the key against the partition.
    #[error("Unable to validate key")]
    CheckKey,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`ValidateHandler::execute`] with a valid key.
    #[test]
    fn validate_handler_execute() {
        // Arrange
        let services = ServiceBuilder::new().with_test_services().build();
        let handler = services.expect::<ValidateHandler>();
        // Act
        let output = handler.execute();
        // Assert
        assert!(output.is_ok());
    }

    /// [`ValidateHandler::execute`] with an invalid key.
    #[test]
    fn validate_handler_execute_invalid_key() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(CheckKeyStub {
                result: Err(CheckKeyError::Incorrect),
            })
            .build();
        let handler = services.expect::<ValidateHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to validate an invalid key");
        assert_eq!(report.current_context(), &ValidateError::CheckKey);
    }

    /// [`ValidateHandler::execute`] with no key sources configured.
    #[test]
    fn validate_handler_execute_no_key_sources() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(Options::fixture_without_key_sources())
            .build();
        let handler = services.expect::<ValidateHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to validate without a key");
        assert_eq!(report.current_context(), &ValidateError::GetKey);
    }
}
