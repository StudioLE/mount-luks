//! Handler for the `set-luks` subcommand.

use crate::prelude::*;

const TOTAL_STEPS: usize = 6;

/// Handler for adding a new key to a LUKS partition.
#[derive(FromServices)]
pub struct SetLuksHandler {
    /// Adapter for checking root privileges.
    is_root: Arc<dyn IsRoot>,
    /// Adapter for checking if a partition is LUKS encrypted.
    is_luks: Arc<dyn IsLuks>,
    /// Adapter for checking if a key is valid for a LUKS partition.
    check_key: Arc<dyn CheckKey>,
    /// Adapter for adding a key to a LUKS partition.
    add_key: Arc<dyn AddKey>,
    /// Adapter for retrieving the composite key.
    get_key: Arc<dyn GetKey>,
    /// Adapter for prompting for a password.
    prompt: Arc<dyn PromptPassword>,
    /// Adapter for resolving PARTUUID to a device path.
    resolve_partition: Arc<dyn ResolvePartition>,
}

impl SetLuksHandler {
    /// Execute the set-luks workflow.
    pub fn execute(&self) -> Result<(), Report<SetLuksError>> {
        let mut progress = Progress::new(TOTAL_STEPS);

        progress.step("Checking if root");
        self.is_root
            .is_root()
            .change_context(SetLuksError::CheckRoot)?;
        progress.ok("Access granted");

        progress.step("Resolving partition");
        let partition = self
            .resolve_partition
            .resolve_partition()
            .change_context(SetLuksError::ResolvePartition)?;
        progress.ok(&format!("Resolved to {}", partition.display()));

        progress.step("Checking if partition is encrypted with LUKS");
        self.is_luks
            .is_luks(&partition)
            .change_context(SetLuksError::CheckLuks)?;
        progress.ok("Partition is encrypted with LUKS");

        progress.step("Getting new key");
        let new_key = self.get_key.get().change_context(SetLuksError::GetKey)?;
        progress.ok("New key retrieved");

        progress.step("Checking if key already exists");
        match self.check_key.check_key(&partition, &new_key) {
            Ok(()) => {
                return Err(Report::new(SetLuksError::KeyExists).attach_path(&partition));
            }
            Err(report) if report.current_context() == &CheckKeyError::Incorrect => {}
            Err(report) => return Err(report.change_context(SetLuksError::CheckKey)),
        }
        progress.ok("Key does not already exist");

        progress.step("Adding new key to partition");
        let existing_passphrase = self
            .prompt
            .prompt("Enter existing passphrase: ")
            .change_context(SetLuksError::Prompt)?;
        self.add_key
            .add_key(&partition, existing_passphrase.trim(), &new_key)
            .change_context(SetLuksError::AddKey)?;
        progress.ok("New key added to partition");

        Ok(())
    }
}

/// Errors returned by [`SetLuksHandler`].
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum SetLuksError {
    /// Unable to confirm the process is running as root.
    #[error("Unable to confirm root privileges")]
    CheckRoot,
    /// Unable to resolve the PARTUUID to a device path.
    #[error("Unable to resolve partition")]
    ResolvePartition,
    /// Unable to confirm the partition is encrypted with LUKS.
    #[error("Unable to confirm partition is encrypted with LUKS")]
    CheckLuks,
    /// Unable to get the new key.
    #[error("Unable to get new key")]
    GetKey,
    /// Unable to check whether the key is already on the partition.
    #[error("Unable to check if key already exists")]
    CheckKey,
    /// The key already unlocks the partition.
    #[error("Key already exists on this partition")]
    KeyExists,
    /// Unable to read the existing passphrase.
    #[error("Unable to read existing passphrase")]
    Prompt,
    /// Unable to add the key to the partition.
    #[error("Unable to add key")]
    AddKey,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// [`SetLuksHandler::execute`] when the key isn't on the partition yet.
    #[test]
    fn set_luks_handler_execute() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(CheckKeyStub {
                result: Err(CheckKeyError::Incorrect),
            })
            .build();
        let handler = services.expect::<SetLuksHandler>();
        // Act
        let output = handler.execute();
        // Assert
        assert!(output.is_ok());
    }

    /// [`SetLuksHandler::execute`] when the key is already on the partition.
    #[test]
    fn set_luks_handler_execute_key_exists() {
        // Arrange
        let services = ServiceBuilder::new().with_test_services().build();
        let handler = services.expect::<SetLuksHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to add an existing key");
        assert_eq!(report.current_context(), &SetLuksError::KeyExists);
    }

    /// [`SetLuksHandler::execute`] when the partition isn't encrypted with LUKS.
    #[test]
    fn set_luks_handler_execute_not_luks() {
        // Arrange
        let services = ServiceBuilder::new()
            .with_test_services()
            .with_instance(IsLuksStub {
                result: Err(IsLuksError::NotLuks),
            })
            .build();
        let handler = services.expect::<SetLuksHandler>();
        // Act
        let output = handler.execute();
        // Assert
        let report = output.expect_err("should not be able to add a key without LUKS");
        assert_eq!(report.current_context(), &SetLuksError::CheckLuks);
    }
}
