//! Handler for the `set-luks` subcommand.

use crate::prelude::*;

/// Handler for adding a new key to a LUKS partition.
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

impl FromServices for SetLuksHandler {
    type Error = ResolveError;
    fn from_services(services: &ServiceProvider) -> Result<Self, Report<ResolveError>> {
        Ok(Self {
            is_root: services.get_trait::<dyn IsRoot>()?,
            is_luks: services.get_trait::<dyn IsLuks>()?,
            check_key: services.get_trait::<dyn CheckKey>()?,
            add_key: services.get_trait::<dyn AddKey>()?,
            get_key: services.get_trait::<dyn GetKey>()?,
            prompt: services.get_trait::<dyn PromptPassword>()?,
            resolve_partition: services.get_trait::<dyn ResolvePartition>()?,
        })
    }
}

impl SetLuksHandler {
    /// Execute the set-luks workflow.
    pub fn execute(&self) -> Result<(), StructuredError> {
        let counter = Mutex::new(0);
        let total_steps = 6;

        print_step_start(&counter, total_steps, "Checking if root");
        self.is_root.is_root().map_err(StructuredError::from)?;
        print_step_completed("Access granted");

        print_step_start(&counter, total_steps, "Resolving partition");
        let partition = self
            .resolve_partition
            .resolve_partition()
            .map_err(StructuredError::from)?;
        print_step_completed(&format!("Resolved to {}", partition.display()));

        print_step_start(
            &counter,
            total_steps,
            "Checking if partition is encrypted with LUKS",
        );
        self.is_luks
            .is_luks(&partition)
            .map_err(StructuredError::from)?;
        print_step_completed("Partition is encrypted with LUKS");

        print_step_start(&counter, total_steps, "Getting new key");
        let new_key = self.get_key.get().map_err(StructuredError::from)?;
        print_step_completed("New key retrieved");

        print_step_start(&counter, total_steps, "Checking if key already exists");
        if self.check_key.check_key(&partition, &new_key).is_ok() {
            return Err(Report::new(KeyAlreadyExists).attach_path(&partition).into());
        }
        print_step_completed("Key does not already exist");

        print_step_start(&counter, total_steps, "Adding new key to partition");
        let existing_passphrase = self
            .prompt
            .prompt("Enter existing passphrase: ")
            .map_err(StructuredError::from)?;
        self.add_key
            .add_key(&partition, existing_passphrase.trim(), &new_key)
            .map_err(StructuredError::from)?;
        print_step_completed("New key added to partition");

        Ok(())
    }
}

/// Error returned by [`SetLuksHandler::execute`] when the key is already present.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("Key already exists on this partition")]
pub struct KeyAlreadyExists;

#[cfg(test)]
impl SetLuksHandler {
    /// Create a [`SetLuksHandler`] with all adapters succeeding by default.
    pub fn mock() -> Self {
        let mut is_root = MockIsRoot::new();
        is_root.expect_is_root().returning(|| Ok(()));
        let mut is_luks = MockIsLuks::new();
        is_luks.expect_is_luks().returning(|_| Ok(()));
        let mut check_key = MockCheckKey::new();
        check_key
            .expect_check_key()
            .returning(|_, _| Err(Report::new(CheckKeyError)));
        let mut add_key = MockAddKey::new();
        add_key.expect_add_key().returning(|_, _, _| Ok(()));
        let mut get_key = MockGetKey::new();
        get_key
            .expect_get()
            .returning(|| Ok(String::from("new-key")));
        let mut prompt = MockPromptPassword::new();
        prompt
            .expect_prompt()
            .returning(|_| Ok(String::from("passphrase")));
        let mut resolve_partition = MockResolvePartition::new();
        resolve_partition
            .expect_resolve_partition()
            .returning(|| Ok(PathBuf::from("/dev/fake-partition")));
        Self {
            is_root: Arc::new(is_root),
            is_luks: Arc::new(is_luks),
            check_key: Arc::new(check_key),
            add_key: Arc::new(add_key),
            get_key: Arc::new(get_key),
            prompt: Arc::new(prompt),
            resolve_partition: Arc::new(resolve_partition),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_luks_succeeds() {
        // Arrange
        let handler = SetLuksHandler::mock();

        // Act
        let result = handler.execute();

        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn set_luks_fails_when_key_exists() {
        // Arrange
        let mut check_key = MockCheckKey::new();
        check_key.expect_check_key().returning(|_, _| Ok(()));
        let handler = SetLuksHandler {
            check_key: Arc::new(check_key),
            ..SetLuksHandler::mock()
        };

        // Act
        let result = handler.execute();

        // Assert
        let error = result.expect_err("should be an error");
        let error = error
            .current_context()
            .downcast_ref::<KeyAlreadyExists>()
            .expect("should be able to downcast");
        assert_eq!(error, &KeyAlreadyExists);
    }

    #[test]
    fn set_luks_fails_when_not_luks() {
        // Arrange
        let mut is_luks = MockIsLuks::new();
        is_luks
            .expect_is_luks()
            .returning(|_| Err(Report::new(IsLuksError::NotLuks)));
        let handler = SetLuksHandler {
            is_luks: Arc::new(is_luks),
            ..SetLuksHandler::mock()
        };

        // Act
        let result = handler.execute();

        // Assert
        let error = result.expect_err("should be an error");
        let error = error
            .current_context()
            .downcast_ref::<IsLuksError>()
            .expect("should be able to downcast");
        assert_eq!(error, &IsLuksError::NotLuks);
    }
}
